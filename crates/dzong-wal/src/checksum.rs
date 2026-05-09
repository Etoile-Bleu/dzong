use crc32fast::Hasher;

/// Calculates the CRC32 checksum of the provided data.
pub fn calculate(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_consistency() {
        let data = b"dzong-wal-test";
        let c1 = calculate(data);
        let c2 = calculate(data);
        assert_eq!(c1, c2);
        assert_ne!(c1, 0);
    }
}
