pub mod checksum;
pub mod reader;
pub mod record;
pub mod writer;

pub use reader::WalReader;
pub use record::{WalOp, WalRecord};
pub use writer::WalWriter;

#[cfg(test)]
mod tests {
    use super::*;
    use dzong_common::{Key, Result, SequenceNumber, Value};
    use std::io::{Read, Seek, SeekFrom, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_wal_roundtrip() -> Result<()> {
        let temp = NamedTempFile::new()?;
        let mut writer = WalWriter::open(temp.path())?;

        let r1 = WalRecord {
            op: WalOp::Put,
            lsn: SequenceNumber::new(1),
            key: Key::new(&b"k1"[..]),
            value: Some(Value::new(&b"v1"[..])),
        };
        let r2 = WalRecord {
            op: WalOp::Delete,
            lsn: SequenceNumber::new(2),
            key: Key::new(&b"k2"[..]),
            value: None,
        };

        writer.append(&r1)?;
        writer.append(&r2)?;
        writer.flush()?;

        let mut reader = WalReader::open(temp.path())?;
        assert_eq!(reader.next_record()?.unwrap(), r1);
        assert_eq!(reader.next_record()?.unwrap(), r2);
        assert!(reader.next_record()?.is_none());

        Ok(())
    }

    #[test]
    fn test_wal_corruption_detection() -> Result<()> {
        let mut temp = NamedTempFile::new()?;
        let mut writer = WalWriter::open(temp.path())?;

        let r1 = WalRecord {
            op: WalOp::Put,
            lsn: SequenceNumber::new(1),
            key: Key::new(&b"k1"[..]),
            value: Some(Value::new(&b"v1"[..])),
        };
        writer.append(&r1)?;
        writer.flush()?;

        // Corrupt the last byte (checksum)
        temp.seek(SeekFrom::End(-1))?;
        let mut last_byte = [0u8; 1];
        temp.read_exact(&mut last_byte)?;
        temp.seek(SeekFrom::End(-1))?;
        temp.write_all(&[!last_byte[0]])?;

        let mut reader = WalReader::open(temp.path())?;
        let result = reader.next_record();
        assert!(result.is_err());

        Ok(())
    }
}
