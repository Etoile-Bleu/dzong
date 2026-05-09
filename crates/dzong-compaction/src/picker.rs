use dzong_manifest::Version;
use std::path::PathBuf;

pub struct CompactionJob {
    pub from_level: u32,
    pub to_level: u32,
    pub input_files: Vec<PathBuf>,
}

pub struct CompactionPicker {
    l0_threshold: usize,
}

impl CompactionPicker {
    pub fn new(l0_threshold: usize) -> Self {
        Self { l0_threshold }
    }

    pub fn pick_compaction(&self, version: &Version) -> Option<CompactionJob> {
        // Check L0
        if version.levels[0].len() >= self.l0_threshold {
            let input_files = version.levels[0]
                .iter()
                .map(|f| f.path.clone())
                .collect();
            
            return Some(CompactionJob {
                from_level: 0,
                to_level: 1,
                input_files,
            });
        }

        // Future: Check other levels for size-tiered or leveled compaction
        None
    }
}
