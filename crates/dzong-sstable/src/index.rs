use dzong_common::{Key, Result};
use std::io::{Read, Write};

/// An entry in the SSTable index.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub first_key: Key,
    pub offset: u64,
    pub block_size: u32,
}

impl IndexEntry {
    pub fn encode<W: Write>(&self, mut writer: W) -> Result<usize> {
        let mut size = 0;
        let key_bytes = self.first_key.as_slice();
        writer.write_all(&(key_bytes.len() as u32).to_le_bytes())?;
        size += 4;
        writer.write_all(key_bytes)?;
        size += key_bytes.len();
        writer.write_all(&self.offset.to_le_bytes())?;
        size += 8;
        writer.write_all(&self.block_size.to_le_bytes())?;
        size += 4;
        Ok(size)
    }

    pub fn decode<R: Read>(mut reader: R) -> Result<Self> {
        let mut key_len_buf = [0u8; 4];
        reader.read_exact(&mut key_len_buf)?;
        let key_len = u32::from_le_bytes(key_len_buf) as usize;
        let mut key_bytes = vec![0u8; key_len];
        reader.read_exact(&mut key_bytes)?;

        let mut offset_buf = [0u8; 8];
        reader.read_exact(&mut offset_buf)?;
        let offset = u64::from_le_bytes(offset_buf);

        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf)?;
        let block_size = u32::from_le_bytes(size_buf);

        Ok(Self {
            first_key: Key::new(key_bytes),
            offset,
            block_size,
        })
    }
}

/// The SSTable index containing entries for all blocks.
#[derive(Default, Clone)]
pub struct Index {
    pub entries: Vec<IndexEntry>,
}

impl Index {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: IndexEntry) {
        self.entries.push(entry);
    }

    /// Finds the block that might contain the key using binary search.
    /// Returns the IndexEntry for the block.
    pub fn find_block(&self, key: &Key) -> Option<&IndexEntry> {
        if self.entries.is_empty() {
            return None;
        }

        // Find the last block whose first_key is <= key.
        let mut left = 0;
        let mut right = self.entries.len() - 1;
        let mut found = None;

        while left <= right {
            let mid = left + (right - left) / 2;
            if &self.entries[mid].first_key <= key {
                found = Some(&self.entries[mid]);
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
        }

        found
    }
}
