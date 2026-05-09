use dzong_testing::TestHarness;
use dzong_common::{Key, Result, Value};

#[test]
fn test_wal_recovery_after_crash() -> Result<()> {
    let mut harness = TestHarness::new()?;
    
    // Insert data but DON'T flush (it stays in WAL and MemTable)
    for i in 0..1000 {
        harness.put(Key::new(format!("wal:{}", i)), Value::new(format!("val:{}", i)))?;
    }
    
    // Simulate crash
    harness.kill()?;
    
    // Recovery should replay WAL
    harness.assert_state()?;
    
    Ok(())
}

#[test]
fn test_sstable_recovery() -> Result<()> {
    let mut harness = TestHarness::new()?;
    
    // Insert enough data to trigger multiple flushes
    // Default max_memtable_size is 1000 in my Options::new
    harness.put_batch(3000, "sst")?;
    
    // Force a restart
    harness.restart()?;
    
    // Data should be in SSTables
    harness.assert_state()?;
    
    Ok(())
}
