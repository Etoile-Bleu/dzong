use crate::record::VersionEdit;
use dzong_common::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;

pub struct ManifestWriter {
    file: File,
}

impl ManifestWriter {
    pub fn open(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self { file })
    }

    pub fn append(&mut self, edit: &VersionEdit) -> Result<()> {
        let json = serde_json::to_string(edit).map_err(|e| {
            dzong_common::DzongError::Codec {
                message: format!("Failed to serialize VersionEdit: {}", e),
                source_context: None,
            }
        })?;
        self.file.write_all(json.as_bytes())?;
        self.file.write_all(b"\n")?;
        self.file.sync_all()?;
        Ok(())
    }
}

pub struct ManifestReader {
    reader: BufReader<File>,
}

impl ManifestReader {
    pub fn open(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    pub fn read_all(&mut self) -> Result<Vec<VersionEdit>> {
        let mut edits = Vec::new();
        for line in self.reader.by_ref().lines() {
            let line: String = line?;
            if line.trim().is_empty() {
                continue;
            }
            let edit: VersionEdit = serde_json::from_str(&line).map_err(|e| {
                dzong_common::DzongError::Corruption {
                    message: format!("Failed to deserialize VersionEdit: {}", e),
                    file_id: None,
                    offset: None,
                }
            })?;
            edits.push(edit);
        }
        Ok(edits)
    }
}
