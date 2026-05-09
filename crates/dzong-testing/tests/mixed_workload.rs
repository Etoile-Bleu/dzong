use dzong_common::Result;
use dzong_core::Options;
use dzong_testing::TestHarness;

#[test]
fn test_mixed_workload_long_run() -> Result<()> {
    let mut options = Options::new("");
    options.max_memtable_size = 500;
    options.l0_compaction_threshold = 4;

    let mut harness = TestHarness::with_options(options)?;

    // Run 10k random operations with fixed seed
    harness.random_ops(12345, 10000)?;

    harness.assert_state()?;

    // Restart mid-way
    harness.restart()?;
    harness.assert_state()?;

    // More ops
    harness.random_ops(67890, 5000)?;
    harness.assert_state()?;

    Ok(())
}
