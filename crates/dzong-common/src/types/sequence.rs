/// Represents a monotonic sequence number for database operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    pub const fn new(seq: u64) -> Self {
        Self(seq)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Represents a Log Sequence Number for WAL operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LSN(u64);

impl LSN {
    pub const fn new(lsn: u64) -> Self {
        Self(lsn)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_values() {
        let seq = SequenceNumber::new(123);
        assert_eq!(seq.as_u64(), 123);

        let lsn = LSN::new(456);
        assert_eq!(lsn.as_u64(), 456);
    }
}
