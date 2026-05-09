use crate::error::Result;

/// A trait for types that can be encoded into a binary format.
pub trait Encode {
    /// Encodes the object into the provided buffer.
    fn encode(&self, dst: &mut Vec<u8>) -> Result<()>;

    /// Returns the exact size of the encoded object in bytes.
    fn encoded_size(&self) -> usize;
}

/// A trait for types that can be decoded from a binary format.
pub trait Decode: Sized {
    /// Decodes the object from the provided buffer.
    fn decode(src: &[u8]) -> Result<(Self, usize)>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::DzongError;

    struct MockType(u32);

    impl Encode for MockType {
        fn encode(&self, dst: &mut Vec<u8>) -> Result<()> {
            dst.extend_from_slice(&self.0.to_be_bytes());
            Ok(())
        }

        fn encoded_size(&self) -> usize {
            4
        }
    }

    impl Decode for MockType {
        fn decode(src: &[u8]) -> Result<(Self, usize)> {
            if src.len() < 4 {
                return Err(DzongError::Codec {
                    message: "Insufficient bytes".to_string(),
                    source_context: None,
                });
            }
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&src[..4]);
            Ok((Self(u32::from_be_bytes(bytes)), 4))
        }
    }

    #[test]
    fn test_codec_mock() {
        let val = MockType(0xDEADBEEF);
        let mut buf = Vec::new();
        val.encode(&mut buf).unwrap();
        assert_eq!(buf.len(), 4);

        let (decoded, size) = MockType::decode(&buf).unwrap();
        assert_eq!(decoded.0, 0xDEADBEEF);
        assert_eq!(size, 4);
    }
}
