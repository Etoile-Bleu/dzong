use crate::memtable::MemTable;
use crate::options::Options;
use dzong_common::{Key, Result, SequenceNumber, Value};
use dzong_manifest::{FileMetadata, VersionEdit};
use dzong_wal::{WalOp, WalReader, WalRecord, WalWriter};
use std::fs;
use tracing::{info, warn};

use std::collections::HashMap;
use std::cell::RefCell;

/// The central execution core of the Dzong database engine.
pub struct DzongEngine {
    wal: WalWriter,
    memtable: MemTable,
    version_set: dzong_manifest::VersionSet,
    manifest_writer: dzong_manifest::ManifestWriter,
    reader_cache: RefCell<HashMap<u64, dzong_sstable::SstableReader>>,
    next_lsn: SequenceNumber,
    options: Options,
}

impl DzongEngine {
    /// Opens a Dzong engine with the specified options.
    /// Performs recovery if a WAL file exists.
    pub fn open(options: Options) -> Result<Self> {
        if !options.data_dir.exists() {
            fs::create_dir_all(&options.data_dir)?;
        }

        let wal_path = options.data_dir.join("wal.log");
        let mut memtable = MemTable::new();
        let mut max_lsn = 0;

        // Recovery (Idempotent)
        if wal_path.exists() {
            info!("Starting recovery from WAL: {:?}", wal_path);
            let mut reader = WalReader::open(&wal_path)?;
            while let Some(record) = reader.next_record()? {
                let lsn_val = record.lsn.as_u64();
                if lsn_val > max_lsn {
                    max_lsn = lsn_val;
                }

                match record.op {
                    WalOp::Put => {
                        if let Some(val) = record.value {
                            memtable.put(record.key, val);
                        } else {
                            warn!("WAL Put record missing value at LSN {}", lsn_val);
                        }
                    }
                    WalOp::Delete => {
                        memtable.delete(record.key);
                    }
                }
            }
        }

        let wal = WalWriter::open(&wal_path)?;
        let next_lsn = SequenceNumber::new(max_lsn + 1);

        // Manifest Recovery
        let manifest_path = options.data_dir.join("MANIFEST-000001");
        let mut version_set = dzong_manifest::VersionSet::new();
        
        if manifest_path.exists() {
            let mut reader = dzong_manifest::ManifestReader::open(manifest_path.clone())?;
            let edits = reader.read_all()?;
            for edit in edits {
                version_set.apply_edit(edit);
            }
        }
        
        let manifest_writer = dzong_manifest::ManifestWriter::open(manifest_path)?;

        Ok(Self {
            wal,
            memtable,
            version_set,
            manifest_writer,
            reader_cache: RefCell::new(HashMap::new()),
            next_lsn,
            options,
        })
    }

    /// Inserts or updates a key-value pair.
    pub fn put(&mut self, key: Key, value: Value) -> Result<()> {
        let lsn = self.assign_lsn();

        let record = WalRecord {
            op: WalOp::Put,
            lsn,
            key: key.clone(),
            value: Some(value.clone()),
        };

        self.wal.append(&record)?;
        self.wal.flush()?;

        self.memtable.put(key, value);
        
        if self.memtable.count() >= self.options.max_memtable_size {
            self.flush_memtable()?;
        }
        Ok(())
    }

    pub fn flush_memtable(&mut self) -> Result<()> {
        info!("Flushing MemTable...");
        let new_id = self.version_set.next_file_id();
        let sst_path = self.options.data_dir.join(format!("{:06}.sst", new_id));
        
        let mut writer = dzong_sstable::SstableWriter::new(&sst_path)?;
        let mut min_key = None;
        let mut max_key = None;

        for (key, val_opt) in self.memtable.iter() {
            let record = dzong_sstable::SstableRecord {
                op: if val_opt.is_some() { dzong_sstable::SstableOp::Put } else { dzong_sstable::SstableOp::Delete },
                lsn: 0, // TODO: Use real LSN from MemTable if stored
                key: key.clone(),
                value: val_opt.clone(),
            };
            
            if min_key.is_none() {
                min_key = Some(key.clone());
            }
            max_key = Some(key.clone());
            writer.add(&record)?;
        }
        writer.finish()?;

        let edit = VersionEdit::AddFile {
            level: 0,
            metadata: FileMetadata {
                id: new_id,
                path: sst_path.clone(),
                min_key: min_key.unwrap_or_else(|| dzong_common::Key::new(&b""[..])),
                max_key: max_key.unwrap_or_else(|| dzong_common::Key::new(&b""[..])),
                size: std::fs::metadata(&sst_path)?.len(),
            },
        };

        self.manifest_writer.append(&edit)?;
        self.version_set.apply_edit(edit);

        let next_id = self.version_set.next_file_id_value();
        self.manifest_writer.append(&VersionEdit::NextFileId(next_id))?;
        
        // Reset MemTable and WAL
        self.memtable = MemTable::new();
        let wal_path = self.options.data_dir.join("wal.log");
        self.wal = dzong_wal::WalWriter::open_truncate(&wal_path)?;

        // Trigger compaction check
        self.maybe_compact()?;

        Ok(())
    }

    fn maybe_compact(&mut self) -> Result<()> {
        let picker = dzong_compaction::CompactionPicker::new(self.options.l0_compaction_threshold);
        if let Some(job) = picker.pick_compaction(&self.version_set.current()) {
            info!("Starting compaction: L{} -> L{}", job.from_level, job.to_level);
            let worker = dzong_compaction::CompactionWorker::new(self.options.data_dir.clone());
            let edit = worker.run_compaction(job, &mut self.version_set)?;
            
            self.manifest_writer.append(&edit)?;
            self.version_set.apply_edit(edit);
            
            // TODO: Delete old files
        }
        Ok(())
    }

    /// Retrieves the value associated with the key.
    pub fn get(&self, key: &Key) -> Result<Option<Value>> {
        // 1. Search MemTable
        if let Some(val_opt) = self.memtable.find(key) {
            return Ok(val_opt.clone());
        }

        // 2. Search SSTables (from newest to oldest)
        let current_version = self.version_set.current();
        for level in &current_version.levels {
            for file_meta in level.iter().rev() {
                if key >= &file_meta.min_key && key <= &file_meta.max_key {
                    let mut cache = self.reader_cache.borrow_mut();
                    if !cache.contains_key(&file_meta.id) {
                        cache.insert(file_meta.id, dzong_sstable::SstableReader::open(&file_meta.path)?);
                    }
                    let reader = cache.get_mut(&file_meta.id).unwrap();
                    if let Some(record) = reader.get(key)? {
                        if record.op == dzong_sstable::SstableOp::Put {
                            return Ok(record.value);
                        } else {
                            return Ok(None);
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Deletes a key from the database.
    pub fn delete(&mut self, key: Key) -> Result<()> {
        let lsn = self.assign_lsn();

        let record = WalRecord {
            op: WalOp::Delete,
            lsn,
            key: key.clone(),
            value: None,
        };

        self.wal.append(&record)?;
        self.wal.flush()?;

        self.memtable.delete(key);
        Ok(())
    }

    fn assign_lsn(&mut self) -> SequenceNumber {
        let lsn = self.next_lsn;
        self.next_lsn = SequenceNumber::new(lsn.as_u64() + 1);
        lsn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_engine_persistence_and_recovery() -> Result<()> {
        let dir = tempdir()?;
        let options = Options::new(dir.path());

        {
            let mut engine = DzongEngine::open(options.clone())?;
            engine.put(Key::new(&b"k1"[..]), Value::new(&b"v1"[..]))?;
            engine.put(Key::new(&b"k2"[..]), Value::new(&b"v2"[..]))?;
            engine.delete(Key::new(&b"k1"[..]))?;
        }

        // Recovery
        let engine = DzongEngine::open(options)?;
        assert_eq!(engine.get(&Key::new(&b"k1"[..]))?, None);
        assert_eq!(
            engine.get(&Key::new(&b"k2"[..]))?,
            Some(Value::new(&b"v2"[..]))
        );
        assert_eq!(engine.next_lsn.as_u64(), 4); // 3 ops done, next is 4

        Ok(())
    }
}
