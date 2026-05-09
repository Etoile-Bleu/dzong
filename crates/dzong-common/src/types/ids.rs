/// Uniquely identifies a table in the database.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableId(u64);

impl TableId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Uniquely identifies a file (e.g., SSTable or WAL) in the database.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(u64);

impl FileId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Represents a level in the LSM-tree hierarchy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(u32);

impl Level {
    pub const fn new(level: u32) -> Self {
        Self(level)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_values() {
        let tid = TableId::new(42);
        assert_eq!(tid.as_u64(), 42);

        let fid = FileId::new(100);
        assert_eq!(fid.as_u64(), 100);

        let lvl = Level::new(1);
        assert_eq!(lvl.as_u32(), 1);
    }
}
