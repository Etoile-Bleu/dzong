use dzong_common::{Key, Value};
use std::collections::BTreeMap;

/// An in-memory sorted structure representing a slice of the database state.
/// This implementation uses a BTreeMap as a v1 sorted structure.
#[derive(Default)]
pub struct MemTable {
    map: BTreeMap<Key, Option<Value>>,
}

impl MemTable {
    /// Creates a new, empty MemTable.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a key-value pair into the MemTable.
    pub fn put(&mut self, key: Key, value: Value) {
        self.map.insert(key, Some(value));
    }

    /// Inserts a tombstone for the specified key.
    pub fn delete(&mut self, key: Key) {
        self.map.insert(key, None);
    }

    /// Retrieves the value associated with the key.
    /// Returns `None` if the key is missing or if it has been deleted (tombstone).
    pub fn get(&self, key: &Key) -> Option<Value> {
        self.map.get(key).and_then(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memtable_basic() {
        let mut mt = MemTable::new();
        let k1 = Key::new(&b"k1"[..]);
        let v1 = Value::new(&b"v1"[..]);

        mt.put(k1.clone(), v1.clone());
        assert_eq!(mt.get(&k1), Some(v1));

        mt.delete(k1.clone());
        assert_eq!(mt.get(&k1), None);
    }
}
