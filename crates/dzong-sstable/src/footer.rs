use dzong_common::Result;
use std::io::{Read, Write};

/// The SSTable footer containing the offset to the index.
pub struct Footer {
    pub index_offset: u64,
}

impl Footer {
    pub const SIZE: usize = 8;

    pub fn encode<W: Write>(&self, mut writer: W) -> Result<usize> {
        writer.write_all(&self.index_offset.to_le_bytes())?;
        Ok(Self::SIZE)
    }

    pub fn decode<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(Self {
            index_offset: u64::from_le_bytes(buf),
        })
    }
}
