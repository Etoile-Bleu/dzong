use bytes::Bytes;
use std::ops::Deref;

/// A lightweight, zero-cost abstraction for database keys.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(Bytes);

impl Key {
    /// Creates a new Key from the given bytes.
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Self(bytes.into())
    }

    /// Returns a reference to the underlying bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for Key {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<u8>> for Key {
    fn from(v: Vec<u8>) -> Self {
        Self::new(v)
    }
}

/// A lightweight, zero-cost abstraction for database values.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Value(Bytes);

impl Value {
    /// Creates a new Value from the given bytes.
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Self(bytes.into())
    }

    /// Returns a reference to the underlying bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for Value {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_value_deref() {
        let key = Key::new(&b"hello"[..]);
        assert_eq!(&*key, b"hello");

        let val = Value::new(&b"world"[..]);
        assert_eq!(&*val, b"world");
    }

    #[test]
    fn test_key_ordering() {
        let k1 = Key::new(&b"a"[..]);
        let k2 = Key::new(&b"b"[..]);
        assert!(k1 < k2);
    }
}
