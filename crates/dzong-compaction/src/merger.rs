use dzong_common::Result;
use dzong_sstable::SstableRecord;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A wrapper for the iterator and its next item to be used in the min-heap.
struct IterWrapper {
    iter: Box<dyn Iterator<Item = Result<SstableRecord>>>,
    next: Option<SstableRecord>,
}

impl IterWrapper {
    fn new(mut iter: Box<dyn Iterator<Item = Result<SstableRecord>>>) -> Result<Self> {
        let next = iter.next().transpose()?;
        Ok(Self { iter, next })
    }

    fn advance(&mut self) -> Result<()> {
        self.next = self.iter.next().transpose()?;
        Ok(())
    }
}

/// We want a min-heap based on Key, but max-heap based on LSN for tie-breaking.
impl PartialEq for IterWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.next.as_ref().map(|r| &r.key) == other.next.as_ref().map(|r| &r.key)
            && self.next.as_ref().map(|r| r.lsn) == other.next.as_ref().map(|r| r.lsn)
    }
}

impl Eq for IterWrapper {}

impl PartialOrd for IterWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IterWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.next, &other.next) {
            (Some(s), Some(o)) => {
                // Min-heap on key (reverse ordering)
                match o.key.cmp(&s.key) {
                    Ordering::Equal => {
                        // Max-heap on LSN when keys are equal
                        s.lsn.cmp(&o.lsn)
                    }
                    ord => ord,
                }
            }
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }
}

pub struct MergeIterator {
    heap: BinaryHeap<IterWrapper>,
}

impl MergeIterator {
    pub fn new(iters: Vec<Box<dyn Iterator<Item = Result<SstableRecord>>>>) -> Result<Self> {
        let mut heap = BinaryHeap::new();
        for iter in iters {
            let wrapper = IterWrapper::new(iter)?;
            if wrapper.next.is_some() {
                heap.push(wrapper);
            }
        }
        Ok(Self { heap })
    }
}

impl Iterator for MergeIterator {
    type Item = Result<SstableRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop the one with the smallest key and largest LSN
        let mut top = self.heap.pop()?;
        let result = top
            .next
            .take()
            .expect("Heap should only contain items with Some(next)");

        // Peek to see if we have duplicates (same key)
        while let Some(peek) = self.heap.peek() {
            if let Some(peek_rec) = &peek.next {
                if peek_rec.key == result.key {
                    // Duplicate key found. Since we resolved by LSN in the heap,
                    // the current 'result' is already the newest one.
                    // We just need to advance the other iterators that have the same key.
                    let mut other = self.heap.pop().unwrap();
                    if let Err(e) = other.advance() {
                        return Some(Err(e));
                    }
                    if other.next.is_some() {
                        self.heap.push(other);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Advance the one we just popped and put it back if it has more
        if let Err(e) = top.advance() {
            return Some(Err(e));
        }
        if top.next.is_some() {
            self.heap.push(top);
        }

        Some(Ok(result))
    }
}
