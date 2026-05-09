use crate::record::WalRecord;
use dzong_common::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Responsible for appending records to the Write-Ahead Log.
pub struct WalWriter {
    writer: BufWriter<File>,
}

impl WalWriter {
    /// Opens a WAL file for appending. Creates it if it doesn't exist.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = OpenOptions::new().append(true).create(true).open(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    /// Opens a WAL file and truncates it.
    pub fn open_truncate(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    /// Appends a record to the WAL.
    /// Note: This does not guarantee durability until `flush` is called.
    pub fn append(&mut self, record: &WalRecord) -> Result<()> {
        let encoded = record.encode();
        self.writer.write_all(&encoded)?;
        Ok(())
    }

    /// Flushes the buffered data to disk and calls `sync_all` for durability.
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        self.writer.get_ref().sync_all()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::WalOp;
    use dzong_common::{Key, SequenceNumber, Value};
    use tempfile::NamedTempFile;

    #[test]
    fn test_wal_writer_append() -> Result<()> {
        let temp = NamedTempFile::new()?;
        let mut writer = WalWriter::open(temp.path())?;

        let record = WalRecord {
            op: WalOp::Put,
            lsn: SequenceNumber::new(1),
            key: Key::new(&b"k1"[..]),
            value: Some(Value::new(&b"v1"[..])),
        };

        writer.append(&record)?;
        writer.flush()?;

        let metadata = temp.as_file().metadata()?;
        assert!(metadata.len() > 0);
        Ok(())
    }
}
