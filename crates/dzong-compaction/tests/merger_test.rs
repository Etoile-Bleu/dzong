use dzong_common::{Key, Result, Value};
use dzong_sstable::{SstableOp, SstableRecord};
use dzong_compaction::merger::MergeIterator;

#[test]
fn test_merge_iterator_lsn_priority() -> Result<()> {
    let k1 = Key::new(&b"key1"[..]);
    let v1 = Value::new(&b"val1"[..]);
    let v2 = Value::new(&b"val2"[..]);

    // Iter 1: key1 @ LSN 10
    let rec1 = SstableRecord {
        op: SstableOp::Put,
        lsn: 10,
        key: k1.clone(),
        value: Some(v1.clone()),
    };
    let iter1 = Box::new(std::iter::once(Ok(rec1))) as Box<dyn Iterator<Item = Result<SstableRecord>>>;

    // Iter 2: key1 @ LSN 20 (Should win)
    let rec2 = SstableRecord {
        op: SstableOp::Put,
        lsn: 20,
        key: k1.clone(),
        value: Some(v2.clone()),
    };
    let iter2 = Box::new(std::iter::once(Ok(rec2))) as Box<dyn Iterator<Item = Result<SstableRecord>>>;

    let merger = MergeIterator::new(vec![iter1, iter2])?;
    let results: Vec<_> = merger.collect::<Result<Vec<_>>>()?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].lsn, 20);
    assert_eq!(results[0].value, Some(v2));

    Ok(())
}

#[test]
fn test_merge_multiple_keys() -> Result<()> {
    let k1 = Key::new(&b"a"[..]);
    let k2 = Key::new(&b"b"[..]);
    let k3 = Key::new(&b"c"[..]);

    let iter1 = Box::new(vec![
        Ok(SstableRecord { op: SstableOp::Put, lsn: 1, key: k1.clone(), value: None }),
        Ok(SstableRecord { op: SstableOp::Put, lsn: 1, key: k3.clone(), value: None }),
    ].into_iter());

    let iter2 = Box::new(vec![
        Ok(SstableRecord { op: SstableOp::Put, lsn: 1, key: k2.clone(), value: None }),
    ].into_iter());

    let merger = MergeIterator::new(vec![iter1, iter2])?;
    let results: Vec<_> = merger.collect::<Result<Vec<_>>>()?;

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].key, k1);
    assert_eq!(results[1].key, k2);
    assert_eq!(results[2].key, k3);

    Ok(())
}
