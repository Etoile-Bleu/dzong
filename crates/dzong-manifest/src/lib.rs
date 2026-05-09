pub mod log;
pub mod record;
pub mod version;

pub use log::{ManifestReader, ManifestWriter};
pub use record::{FileMetadata, VersionEdit};
pub use version::{Version, VersionSet};

#[cfg(test)]
mod tests {
    use super::*;
    use dzong_common::Key;
    use dzong_common::Result;
    use tempfile::tempdir;

    #[test]
    fn test_manifest_roundtrip() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("MANIFEST-000001");

        let mut writer = ManifestWriter::open(path.clone())?;

        let meta = FileMetadata {
            id: 1,
            path: "000001.sst".into(),
            min_key: Key::new(&b"a"[..]),
            max_key: Key::new(&b"z"[..]),
            size: 1024,
        };

        let edit1 = VersionEdit::AddFile {
            level: 0,
            metadata: meta,
        };
        let edit2 = VersionEdit::NextFileId(2);

        writer.append(&edit1)?;
        writer.append(&edit2)?;

        let mut reader = ManifestReader::open(path)?;
        let edits = reader.read_all()?;

        assert_eq!(edits.len(), 2);
        assert_eq!(edits[0], edit1);
        assert_eq!(edits[1], edit2);

        let mut vs = VersionSet::new();
        for edit in edits {
            vs.apply_edit(edit);
        }

        assert_eq!(vs.current().levels[0].len(), 1);
        assert_eq!(vs.current().levels[0][0].id, 1);

        Ok(())
    }
}
