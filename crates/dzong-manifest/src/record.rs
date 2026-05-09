use dzong_common::Key;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FileMetadata {
    pub id: u64,
    pub path: PathBuf,
    pub min_key: Key,
    pub max_key: Key,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum VersionEdit {
    AddFile { level: u32, metadata: FileMetadata },
    RemoveFile { level: u32, id: u64 },
    NextFileId(u64),
}
