use crate::record::{FileMetadata, VersionEdit};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Version {
    pub levels: Vec<Vec<FileMetadata>>,
}

impl Version {
    pub fn new() -> Self {
        Self {
            levels: vec![vec![]; 7], // Support 7 levels (L0 to L6)
        }
    }

    pub fn apply(&self, edit: &VersionEdit) -> Self {
        let mut new_version = self.clone();
        match edit {
            VersionEdit::AddFile { level, metadata } => {
                new_version.levels[*level as usize].push(metadata.clone());
            }
            VersionEdit::RemoveFile { level, id } => {
                new_version.levels[*level as usize].retain(|f| f.id != *id);
            }
            VersionEdit::NextFileId(_) => {} // Handled by VersionSet
        }
        new_version
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VersionSet {
    current: Arc<Version>,
    next_file_id: u64,
}

impl VersionSet {
    pub fn new() -> Self {
        Self {
            current: Arc::new(Version::new()),
            next_file_id: 1,
        }
    }

    pub fn current(&self) -> Arc<Version> {
        self.current.clone()
    }

    pub fn next_file_id(&mut self) -> u64 {
        let id = self.next_file_id;
        self.next_file_id += 1;
        id
    }

    pub fn next_file_id_value(&self) -> u64 {
        self.next_file_id
    }

    pub fn apply_edit(&mut self, edit: VersionEdit) {
        if let VersionEdit::NextFileId(id) = edit {
            self.next_file_id = id;
        } else {
            let new_version = self.current.apply(&edit);
            self.current = Arc::new(new_version);
        }
    }
}

impl Default for VersionSet {
    fn default() -> Self {
        Self::new()
    }
}
