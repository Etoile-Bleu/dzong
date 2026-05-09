use dzong_common::Result;
use dzong_testing::TestHarness;

#[test]
fn test_basic_sequential_workload() -> Result<()> {
    let mut harness = TestHarness::new()?;

    // 5k sequential writes
    harness.put_batch(5000, "seq")?;

    // Assert all present
    harness.assert_state()?;

    // Restart and assert again
    harness.restart()?;
    harness.assert_state()?;

    Ok(())
}

#[test]
fn test_random_read_hits() -> Result<()> {
    let mut harness = TestHarness::new()?;

    // Write some data
    harness.put_batch(100, "hit")?;

    // Random operations (60% put, 20% get, 20% delete)
    harness.random_ops(42, 1000)?;

    harness.assert_state()?;

    Ok(())
}
