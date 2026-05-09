use crate::block::BlockBuilder;
use crate::index::{Index, IndexEntry};
use crate::record::{SstableOp, SstableRecord};
use crate::footer::Footer;
use dzong_common::{Key, Result, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const BLOCK_SIZE_THRESHOLD: usize = 4096;

pub struct SstableWriter {
    writer: BufWriter<File>,
    index: Index,
    offset: u64,
    current_builder: Option<BlockBuilder>,
}

impl SstableWriter {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
            index: Index::new(),
            offset: 0,
            current_builder: Some(BlockBuilder::new()),
        })
    }

    pub fn write_from_memtable(path: &Path, memtable: &BTreeMap<Key, Option<Value>>) -> Result<()> {
        let mut writer = Self::new(path)?;

        for (key, value) in memtable {
            let record = SstableRecord {
                op: if value.is_some() { SstableOp::Put } else { SstableOp::Delete },
                lsn: 0, // Placeholder for v1
                key: key.clone(),
                value: value.clone(),
            };
            writer.add(&record)?;
        }

        writer.finish()?;
        Ok(())
    }

    pub fn add(&mut self, record: &SstableRecord) -> Result<()> {
        let mut builder = self.current_builder.take().unwrap();
        
        if builder.size() >= BLOCK_SIZE_THRESHOLD {
            self.finish_block(builder)?;
            builder = BlockBuilder::new();
        }
        
        builder.add(record)?;
        self.current_builder = Some(builder);
        Ok(())
    }

    pub fn finish(&mut self) -> Result<()> {
        if let Some(builder) = self.current_builder.take() {
            if !builder.is_empty() {
                self.finish_block(builder)?;
            }
        }
        self.write_index_and_footer()?;
        Ok(())
    }

    fn finish_block(&mut self, builder: BlockBuilder) -> Result<()> {
        let first_key = builder.first_key().ok_or_else(|| {
            dzong_common::DzongError::Invariant {
                message: "Attempted to finish an empty block".to_string(),
                context: None,
            }
        })?;
        let data = builder.build();
        let size = data.len() as u32;
        
        self.writer.write_all(&data)?;
        
        self.index.add(IndexEntry {
            first_key,
            offset: self.offset,
            block_size: size,
        });

        self.offset += size as u64;
        Ok(())
    }

    fn write_index_and_footer(&mut self) -> Result<()> {
        let index_offset = self.offset;
        for entry in &self.index.entries {
            entry.encode(&mut self.writer)?;
        }

        let footer = Footer { index_offset };
        footer.encode(&mut self.writer)?;
        self.writer.flush()?;
        Ok(())
    }
}

// I need to update BlockBuilder to store the first key.
