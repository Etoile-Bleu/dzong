/// A trait for types that can report their in-memory or on-disk size.
pub trait SizeOf {
    /// Returns the size in bytes.
    fn size_of(&self) -> usize;
}

impl SizeOf for Vec<u8> {
    fn size_of(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(v.size_of(), 3);
    }
}
