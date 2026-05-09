use dzong_common::{Key, Result, Value};
use std::io::{Read, Write};

/// Operation type stored in the SSTable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SstableOp {
    Put = 1,
    Delete = 2,
}

impl SstableOp {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(SstableOp::Put),
            2 => Some(SstableOp::Delete),
            _ => None,
        }
    }
}

/// A physical record in an SSTable.
#[derive(Debug, Clone)]
pub struct SstableRecord {
    pub op: SstableOp,
    pub lsn: u64,
    pub key: Key,
    pub value: Option<Value>,
}

impl SstableRecord {
    pub fn encode<W: Write>(&self, mut writer: W) -> Result<usize> {
        let mut size = 0;
        writer.write_all(&[self.op as u8])?;
        size += 1;
        writer.write_all(&self.lsn.to_le_bytes())?;
        size += 8;

        let key_bytes = self.key.as_slice();
        writer.write_all(&(key_bytes.len() as u32).to_le_bytes())?;
        size += 4;
        writer.write_all(key_bytes)?;
        size += key_bytes.len();

        let val_bytes = self.value.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        writer.write_all(&(val_bytes.len() as u32).to_le_bytes())?;
        size += 4;
        writer.write_all(val_bytes)?;
        size += val_bytes.len();

        Ok(size)
    }

    pub fn decode<R: Read>(mut reader: R) -> Result<Self> {
        let mut op_buf = [0u8; 1];
        reader.read_exact(&mut op_buf)?;
        let op =
            SstableOp::from_u8(op_buf[0]).ok_or_else(|| dzong_common::DzongError::Corruption {
                message: format!("Invalid SstableOp: {}", op_buf[0]),
                file_id: None,
                offset: None,
            })?;

        let mut lsn_buf = [0u8; 8];
        reader.read_exact(&mut lsn_buf)?;
        let lsn = u64::from_le_bytes(lsn_buf);

        let mut key_len_buf = [0u8; 4];
        reader.read_exact(&mut key_len_buf)?;
        let key_len = u32::from_le_bytes(key_len_buf) as usize;
        let mut key_bytes = vec![0u8; key_len];
        reader.read_exact(&mut key_bytes)?;

        let mut val_len_buf = [0u8; 4];
        reader.read_exact(&mut val_len_buf)?;
        let val_len = u32::from_le_bytes(val_len_buf) as usize;
        let mut val_bytes = vec![0u8; val_len];
        reader.read_exact(&mut val_bytes)?;

        let value = if op == SstableOp::Delete {
            None
        } else {
            Some(Value::new(val_bytes))
        };

        Ok(Self {
            op,
            lsn,
            key: Key::new(key_bytes),
            value,
        })
    }
}
