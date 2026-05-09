use dzong_common::{Key, Result, Value};
use dzong_core::Options;
use dzong_testing::TestHarness;

#[test]
fn test_compaction_deduplication_stress() -> Result<()> {
    let mut options = Options::new(""); // Path will be set by harness
    options.max_memtable_size = 100;
    options.l0_compaction_threshold = 2; // Very aggressive compaction

    let mut harness = TestHarness::with_options(options)?;

    // Overwrite the same keys many times to create many versions in many SSTables
    let keys: Vec<Key> = (0..10).map(|i| Key::new(format!("key:{}", i))).collect();

    for cycle in 0..50 {
        for key in &keys {
            harness.put(key.clone(), Value::new(format!("cycle:{}", cycle)))?;
        }
    }

    // Many flushes and compactions should have happened
    harness.assert_state()?;

    // Restart to ensure manifest and new files are correct
    harness.restart()?;
    harness.assert_state()?;

    Ok(())
}
