use dzong_common::{DzongError, Key, Result, SequenceNumber, Value};

/// Represents the type of operation in a WAL record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalOp {
    Put = 0,
    Delete = 1,
}

impl WalOp {
    pub fn from_u8(v: u8) -> Result<Self> {
        match v {
            0 => Ok(Self::Put),
            1 => Ok(Self::Delete),
            _ => Err(DzongError::Codec {
                message: format!("Unknown WalOp type: {}", v),
                source_context: Some("WalOp::from_u8".to_string()),
            }),
        }
    }
}

/// A single record in the Write-Ahead Log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalRecord {
    pub op: WalOp,
    pub lsn: SequenceNumber,
    pub key: Key,
    pub value: Option<Value>,
}

impl WalRecord {
    /// Encodes the record into a binary format.
    pub fn encode(&self) -> Vec<u8> {
        let key_bytes = self.key.as_slice();
        let val_bytes = self.value.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);

        let payload_len = 1 + 8 + 4 + key_bytes.len() + 4 + val_bytes.len();
        let mut buf = Vec::with_capacity(4 + payload_len + 4);

        // Header: Total length of payload
        buf.extend_from_slice(&(payload_len as u32).to_be_bytes());

        // Payload
        let payload_start = buf.len();
        buf.push(self.op as u8);
        buf.extend_from_slice(&self.lsn.as_u64().to_be_bytes());
        buf.extend_from_slice(&(key_bytes.len() as u32).to_be_bytes());
        buf.extend_from_slice(key_bytes);
        buf.extend_from_slice(&(val_bytes.len() as u32).to_be_bytes());
        buf.extend_from_slice(val_bytes);

        // Checksum
        let checksum = crate::checksum::calculate(&buf[payload_start..]);
        buf.extend_from_slice(&checksum.to_be_bytes());

        buf
    }

    /// Returns the expected size of the encoded record.
    pub fn encoded_size(&self) -> usize {
        let key_len = self.key.as_slice().len();
        let val_len = self.value.as_ref().map(|v| v.as_slice().len()).unwrap_or(0);
        4 + (1 + 8 + 4 + key_len + 4 + val_len) + 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_encoding() {
        let record = WalRecord {
            op: WalOp::Put,
            lsn: SequenceNumber::new(1),
            key: Key::new(&b"k1"[..]),
            value: Some(Value::new(&b"v1"[..])),
        };
        let encoded = record.encode();
        assert_eq!(encoded.len(), record.encoded_size());
    }
}
