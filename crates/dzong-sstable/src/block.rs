use crate::record::SstableRecord;
use dzong_common::{Key, Result};
use std::io::Cursor;

/// Builds a data block by buffering records.
#[derive(Default)]
pub struct BlockBuilder {
    buffer: Vec<u8>,
    count: usize,
    first_key: Option<Key>,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a record to the block.
    pub fn add(&mut self, record: &SstableRecord) -> Result<()> {
        if self.first_key.is_none() {
            self.first_key = Some(record.key.clone());
        }
        record.encode(&mut self.buffer)?;
        self.count += 1;
        Ok(())
    }

    /// Returns the current size of the block in bytes.
    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the number of records in the block.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Finalizes the block and returns the byte content.
    pub fn build(self) -> Vec<u8> {
        self.buffer
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn first_key(&self) -> Option<Key> {
        self.first_key.clone()
    }
}

/// Reads and scans records within a block.
pub struct BlockReader<'a> {
    data: &'a [u8],
}

impl<'a> BlockReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn get(&self, key: &Key) -> Result<Option<SstableRecord>> {
        let mut best_record: Option<SstableRecord> = None;
        let iter = BlockIterator::new(self.data);

        for result in iter {
            let record = result?;
            if &record.key == key {
                match &best_record {
                    None => best_record = Some(record),
                    Some(best) if record.lsn > best.lsn => best_record = Some(record),
                    _ => {}
                }
            }
        }

        Ok(best_record)
    }

    pub fn iter(&self) -> BlockIterator<'a> {
        BlockIterator::new(self.data)
    }
}

pub struct BlockIterator<'a> {
    cursor: Cursor<&'a [u8]>,
    data_len: u64,
}

impl<'a> BlockIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
            data_len: data.len() as u64,
        }
    }
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = Result<SstableRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.position() >= self.data_len {
            return None;
        }
        Some(SstableRecord::decode(&mut self.cursor))
    }
}
