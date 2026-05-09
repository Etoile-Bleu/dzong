use dzong_testing::TestHarness;
use dzong_common::{Result, Key, Value};
use dzong_core::Options;

#[test]
fn test_crash_during_heavy_compaction_load() -> Result<()> {
    let mut options = Options::new("");
    options.max_memtable_size = 50; // Small flushes
    options.l0_compaction_threshold = 2; // Constant compaction
    
    let mut harness = TestHarness::with_options(options)?;
    
    // Insert 1000 keys. Many compactions will be triggered.
    for i in 0..1000 {
        let k = Key::new(format!("chaos:{}", i));
        let v = Value::new(format!("data:{}", i));
        
        // We "kill" every 100 insertions
        if i > 0 && i % 100 == 0 {
            harness.kill()?;
        }
        
        harness.put(k, v)?;
    }
    
    harness.assert_state()?;
    harness.restart()?;
    harness.assert_state()?;
    
    Ok(())
}
