use crate::memtable::MemTable;
use crate::options::Options;
use dzong_common::{Key, Result, SequenceNumber, Value};
use dzong_wal::{WalOp, WalReader, WalRecord, WalWriter};
use std::fs;
use tracing::{info, warn};

/// The central execution core of the Dzong database engine.
pub struct DzongEngine {
    wal: WalWriter,
    memtable: MemTable,
    next_lsn: SequenceNumber,
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

        Ok(Self {
            wal,
            memtable,
            next_lsn,
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
        Ok(())
    }

    /// Retrieves the value associated with the key.
    pub fn get(&self, key: &Key) -> Result<Option<Value>> {
        Ok(self.memtable.get(key))
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
