use dzong_core::{DzongEngine, Options};
use dzong_common::{Key, Result, Value};
use tempfile::TempDir;
use std::collections::BTreeMap;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct TestHarness {
    pub engine: DzongEngine,
    pub data_dir: TempDir,
    pub options: Options,
    pub ground_truth: BTreeMap<Key, Option<Value>>,
}

impl TestHarness {
    pub fn new() -> Result<Self> {
        let dir = TempDir::new()?;
        let options = Options::new(dir.path());
        let engine = DzongEngine::open(options.clone())?;
        
        Ok(Self {
            engine,
            data_dir: dir,
            options,
            ground_truth: BTreeMap::new(),
        })
    }

    pub fn with_options(mut options: Options) -> Result<Self> {
        let dir = TempDir::new()?;
        options.data_dir = dir.path().to_path_buf();
        let engine = DzongEngine::open(options.clone())?;
        
        Ok(Self {
            engine,
            data_dir: dir,
            options,
            ground_truth: BTreeMap::new(),
        })
    }

    pub fn restart(&mut self) -> Result<()> {
        // Simulates a clean restart
        // Drop engine to ensure all handles are closed
        // In a real crash, we wouldn't drop cleanly, but Dzong should handle it via WAL/Manifest.
        let options = self.options.clone();
        // Drop the old engine
        // (Self is not dropped, just the field)
        // We need to re-open it.
        self.engine = DzongEngine::open(options)?;
        Ok(())
    }

    pub fn kill(&mut self) -> Result<()> {
        // Simulates a crash (no clean shutdown)
        // In Rust, we just drop it. If we had background threads, we'd stop them abruptly.
        self.restart()
    }

    pub fn put(&mut self, key: Key, value: Value) -> Result<()> {
        self.engine.put(key.clone(), value.clone())?;
        self.ground_truth.insert(key, Some(value));
        Ok(())
    }

    pub fn delete(&mut self, key: Key) -> Result<()> {
        self.engine.delete(key.clone())?;
        self.ground_truth.insert(key, None);
        Ok(())
    }

    pub fn get(&self, key: &Key) -> Result<Option<Value>> {
        self.engine.get(key)
    }

    pub fn put_batch(&mut self, n: usize, prefix: &str) -> Result<()> {
        for i in 0..n {
            let k = Key::new(format!("{}:{:06}", prefix, i));
            let v = Value::new(format!("val:{}", i));
            self.put(k, v)?;
        }
        Ok(())
    }

    pub fn assert_state(&self) -> Result<()> {
        for (key, expected_val) in &self.ground_truth {
            let actual_val = self.engine.get(key)?;
            match expected_val {
                Some(v) => {
                    assert_eq!(actual_val.as_ref(), Some(v), "Mismatch for key {:?}", key);
                }
                None => {
                    assert_eq!(actual_val, None, "Expected tombstone for key {:?}", key);
                }
            }
        }
        Ok(())
    }

    pub fn random_ops(&mut self, seed: u64, count: usize) -> Result<()> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let keys: Vec<Key> = (0..100).map(|i| Key::new(format!("key:{}", i))).collect();

        for _ in 0..count {
            let key = keys[rng.gen_range(0..keys.len())].clone();
            let op = rng.gen_range(0..100);
            if op < 60 {
                // Put
                let val = Value::new(format!("val:{}", rng.gen::<u64>()));
                self.put(key, val)?;
            } else if op < 80 {
                // Get & Validate
                let expected = self.ground_truth.get(&key).and_then(|v| v.clone());
                let actual = self.get(&key)?;
                assert_eq!(actual, expected);
            } else {
                // Delete
                self.delete(key)?;
            }
        }
        Ok(())
    }
}
