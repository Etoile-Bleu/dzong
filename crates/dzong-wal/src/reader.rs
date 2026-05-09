use crate::record::{WalOp, WalRecord};
use dzong_common::{DzongError, Key, Result, SequenceNumber, Value};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Responsible for reading and replaying records from a Write-Ahead Log.
pub struct WalReader {
    reader: BufReader<File>,
}

impl WalReader {
    /// Opens a WAL file for reading.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    /// Reads the next record from the log. Returns `None` at EOF.
    pub fn next_record(&mut self) -> Result<Option<WalRecord>> {
        let mut len_buf = [0u8; 4];
        if let Err(e) = self.reader.read_exact(&mut len_buf) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                return Ok(None);
            }
            return Err(e.into());
        }

        let payload_len = u32::from_be_bytes(len_buf) as usize;
        let mut payload = vec![0u8; payload_len];
        self.reader.read_exact(&mut payload)?;

        let mut checksum_buf = [0u8; 4];
        self.reader.read_exact(&mut checksum_buf)?;
        let expected_checksum = u32::from_be_bytes(checksum_buf);

        let actual_checksum = crate::checksum::calculate(&payload);
        if actual_checksum != expected_checksum {
            return Err(DzongError::Corruption {
                message: "WAL record checksum mismatch".to_string(),
                file_id: None,
                offset: None, // Could be tracked if needed
            });
        }

        self.parse_record(&payload)
    }

    fn parse_record(&self, payload: &[u8]) -> Result<Option<WalRecord>> {
        if payload.len() < 13 {
            // op(1) + lsn(8) + klen(4)
            return Err(DzongError::Codec {
                message: "WAL record payload too short".to_string(),
                source_context: Some("WalReader::parse_record".to_string()),
            });
        }

        let op = WalOp::from_u8(payload[0])?;
        let lsn = SequenceNumber::new(u64::from_be_bytes(payload[1..9].try_into().unwrap()));
        let klen = u32::from_be_bytes(payload[9..13].try_into().unwrap()) as usize;

        if payload.len() < 13 + klen + 4 {
            return Err(DzongError::Codec {
                message: "WAL record payload truncated".to_string(),
                source_context: Some("WalReader::parse_record".to_string()),
            });
        }

        let key = Key::new(payload[13..13 + klen].to_vec());
        let vlen = u32::from_be_bytes(payload[13 + klen..17 + klen].try_into().unwrap()) as usize;

        if payload.len() != 17 + klen + vlen {
            return Err(DzongError::Codec {
                message: "WAL record payload length mismatch".to_string(),
                source_context: Some("WalReader::parse_record".to_string()),
            });
        }

        let value = if vlen > 0 {
            Some(Value::new(payload[17 + klen..].to_vec()))
        } else {
            None
        };

        Ok(Some(WalRecord {
            op,
            lsn,
            key,
            value,
        }))
    }
}
