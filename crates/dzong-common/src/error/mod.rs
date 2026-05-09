use thiserror::Error;

/// The primary error type for all Dzong operations,
/// providing structured diagnostic context.
#[derive(Debug, Error)]
pub enum DzongError {
    /// Errors related to disk I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors indicating data corruption or integrity failure.
    #[error("Corruption: {message} (file_id: {file_id:?}, offset: {offset:?})")]
    Corruption {
        message: String,
        file_id: Option<u64>,
        offset: Option<u64>,
    },

    /// Errors occurring during serialization or deserialization.
    #[error("Codec error: {message} (source: {source_context:?})")]
    Codec {
        message: String,
        source_context: Option<String>,
    },

    /// Errors indicating that a requested key was not found.
    #[error("Key not found: {key:?}")]
    NotFound { key: Vec<u8> },

    /// Errors indicating a violation of internal invariants.
    #[error("Internal invariant violation: {message} (context: {context:?})")]
    Invariant {
        message: String,
        context: Option<String>,
    },
}

/// A specialized Result type for Dzong operations.
pub type Result<T> = std::result::Result<T, DzongError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_io_propagation() {
        fn fail() -> Result<()> {
            Err(io::Error::other("disk failure").into())
        }
        let res = fail();
        assert!(matches!(res, Err(DzongError::Io(_))));
    }

    #[test]
    fn test_corruption_formatting() {
        let err = DzongError::Corruption {
            message: "invalid block checksum".to_string(),
            file_id: Some(123),
            offset: Some(4096),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("invalid block checksum"));
        assert!(msg.contains("123"));
        assert!(msg.contains("4096"));
    }

    #[test]
    fn test_not_found_context() {
        let err = DzongError::NotFound {
            key: b"user_123".to_vec(),
        };
        if let DzongError::NotFound { key } = err {
            assert_eq!(key, b"user_123");
        } else {
            panic!("expected NotFound variant");
        }
    }
}
