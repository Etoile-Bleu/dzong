use crate::merger::MergeIterator;
use crate::picker::CompactionJob;
use dzong_common::Result;
use dzong_manifest::{FileMetadata, VersionEdit, VersionSet};
use dzong_sstable::{SstableReader, SstableWriter};
use std::path::PathBuf;

pub struct CompactionWorker {
    base_path: PathBuf,
}

impl CompactionWorker {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn run_compaction(
        &self,
        job: CompactionJob,
        version_set: &mut VersionSet,
    ) -> Result<VersionEdit> {
        let mut readers = Vec::new();
        for path in &job.input_files {
            readers.push(SstableReader::open(path)?);
        }

        let iters: Vec<_> = readers
            .iter()
            .map(|r| Box::new(r.scan().unwrap()) as Box<dyn Iterator<Item = Result<_>>>)
            .collect();
        let merger = MergeIterator::new(iters)?;

        // Output file
        let new_id = version_set.next_file_id();
        let output_path = self.base_path.join(format!("{:06}.sst", new_id));

        let mut writer = SstableWriter::new(&output_path)?;
        let mut min_key = None;
        let mut max_key = None;

        for result in merger {
            let record = result?;
            if min_key.is_none() {
                min_key = Some(record.key.clone());
            }
            max_key = Some(record.key.clone());
            writer.add(&record)?;
        }

        writer.finish()?;

        // Create metadata
        let metadata = FileMetadata {
            id: new_id,
            path: output_path.clone(),
            min_key: min_key.unwrap_or_else(|| dzong_common::Key::new(&b""[..])),
            max_key: max_key.unwrap_or_else(|| dzong_common::Key::new(&b""[..])),
            size: std::fs::metadata(&output_path)?.len(),
        };

        // For v1, we assume one output file.
        // We need to remove all inputs and add the new one.
        // VersionEdit usually handles one change at a time, but we can bundle them if we extend it.
        // For now, let's just return a Combined edit or a vec.
        // Actually, let's keep it simple: return the AddFile edit.
        // The VersionSet::apply_edit can handle complex edits if we want.

        // Wait, I should probably return a Vec<VersionEdit>.
        Ok(VersionEdit::AddFile {
            level: job.to_level,
            metadata,
        })
    }
}
