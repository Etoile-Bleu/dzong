use crate::block::BlockReader;
use crate::footer::Footer;
use crate::index::{Index, IndexEntry};
use crate::record::SstableOp;
use dzong_common::{Key, Result, Value};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub struct SstableReader {
    file: File,
    index: Index,
}

impl SstableReader {
    pub fn open(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        
        if file_size < Footer::SIZE as u64 {
            return Err(dzong_common::DzongError::Corruption {
                message: "SSTable file too small for footer".to_string(),
                file_id: None,
                offset: Some(0),
            });
        }

        // Read Footer
        file.seek(SeekFrom::Start(file_size - Footer::SIZE as u64))?;
        let footer = Footer::decode(&mut file)?;

        // Read Index
        if footer.index_offset > file_size - Footer::SIZE as u64 {
            return Err(dzong_common::DzongError::Corruption {
                message: "Invalid index_offset in footer".to_string(),
                file_id: None,
                offset: Some(file_size - Footer::SIZE as u64),
            });
        }

        file.seek(SeekFrom::Start(footer.index_offset))?;
        let mut index = Index::new();
        let index_bytes_to_read = (file_size - Footer::SIZE as u64) - footer.index_offset;
        let mut index_reader = (&file).take(index_bytes_to_read);

        // This is a bit tricky with Take since it doesn't support seek, but we only read forward.
        // But we need to use the same file handle or a clone?
        // Let's just read the whole index into memory for now.
        let mut index_data = vec![0u8; index_bytes_to_read as usize];
        index_reader.read_exact(&mut index_data)?;
        
        let mut cursor = std::io::Cursor::new(index_data);
        while cursor.position() < index_bytes_to_read {
            index.add(IndexEntry::decode(&mut cursor)?);
        }

        Ok(Self { file, index })
    }

    pub fn get(&mut self, key: &Key) -> Result<Option<Value>> {
        let entry = match self.index.find_block(key) {
            Some(e) => e,
            None => return Ok(None),
        };

        // Load block
        self.file.seek(SeekFrom::Start(entry.offset))?;
        let mut block_data = vec![0u8; entry.block_size as usize];
        self.file.read_exact(&mut block_data)?;

        // Scan block
        let block_reader = BlockReader::new(&block_data);
        let record = block_reader.get(key)?;

        match record {
            Some(r) if r.op == SstableOp::Put => Ok(r.value),
            _ => Ok(None), // Delete or not found
        }
    }
}
