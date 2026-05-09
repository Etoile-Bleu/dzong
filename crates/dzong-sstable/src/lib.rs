pub mod block;
pub mod footer;
pub mod index;
pub mod reader;
pub mod record;
pub mod writer;

pub use reader::{SstableIterator, SstableReader};
pub use record::{SstableOp, SstableRecord};
pub use writer::SstableWriter;

use dzong_common::{Key, Result, Value};
use std::collections::BTreeMap;
use std::path::Path;

pub struct Sstable;

impl Sstable {
    pub fn write_from_memtable(path: &Path, memtable: &BTreeMap<Key, Option<Value>>) -> Result<()> {
        SstableWriter::write_from_memtable(path, memtable)
    }

    pub fn get(path: &Path, key: &Key) -> Result<Option<Value>> {
        let mut reader = SstableReader::open(path)?;
        match reader.get(key)? {
            Some(record) if record.op == SstableOp::Put => Ok(record.value),
            _ => Ok(None),
        }
    }
}
