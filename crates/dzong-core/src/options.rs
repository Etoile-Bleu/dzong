use std::path::PathBuf;

/// Configuration options for the Dzong engine.
#[derive(Clone, Debug)]
pub struct Options {
    /// Path to the data directory.
    pub data_dir: PathBuf,
    pub max_memtable_size: usize,
    pub l0_compaction_threshold: usize,
}

impl Options {
    /// Creates a new Options instance with the specified data directory.
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
            max_memtable_size: 1000,
            l0_compaction_threshold: 4,
        }
    }
}
