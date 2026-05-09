use dzong_common::{Key, Result, Value};
use dzong_sstable::Sstable;
use std::collections::BTreeMap;
use tempfile::tempdir;

#[test]
fn test_sstable_basic_roundtrip() -> Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("test.sst");
    let mut memtable = BTreeMap::new();

    let k1 = Key::new(&b"key1"[..]);
    let v1 = Value::new(&b"val1"[..]);
    let k2 = Key::new(&b"key2"[..]);
    let v2 = Value::new(&b"val2"[..]);

    memtable.insert(k1.clone(), Some(v1.clone()));
    memtable.insert(k2.clone(), Some(v2.clone()));

    Sstable::write_from_memtable(&path, &memtable)?;

    assert_eq!(Sstable::get(&path, &k1)?, Some(v1));
    assert_eq!(Sstable::get(&path, &k2)?, Some(v2));
    assert_eq!(Sstable::get(&path, &Key::new(&b"unknown"[..]))?, None);

    Ok(())
}

#[test]
fn test_sstable_with_tombstones() -> Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("tombstone.sst");
    let mut memtable = BTreeMap::new();

    let k1 = Key::new(&b"key1"[..]);

    memtable.insert(k1.clone(), None); // Tombstone

    Sstable::write_from_memtable(&path, &memtable)?;

    assert_eq!(Sstable::get(&path, &k1)?, None);

    Ok(())
}

#[test]
fn test_sstable_multi_block() -> Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("multi_block.sst");
    let mut memtable = BTreeMap::new();

    // Insert 1000 keys to trigger multiple blocks (4KB threshold)
    for i in 0..1000 {
        let key = Key::new(format!("key{:04}", i));
        let val = Value::new(format!("val{:04}", i));
        memtable.insert(key, Some(val));
    }

    Sstable::write_from_memtable(&path, &memtable)?;

    for i in 0..1000 {
        let key = Key::new(format!("key{:04}", i));
        let val = Value::new(format!("val{:04}", i));
        assert_eq!(Sstable::get(&path, &key)?, Some(val));
    }

    Ok(())
}

#[test]
fn test_sstable_corruption() -> Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("corrupt.sst");

    // Write garbage
    std::fs::write(&path, b"garbage data that is not a valid sstable")?;

    let result = Sstable::get(&path, &Key::new(&b"k"[..]));
    assert!(result.is_err());

    Ok(())
}
