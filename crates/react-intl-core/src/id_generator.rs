//! ID generation utilities
//!
//! Provides hash functions for generating message IDs compatible with babel-plugin-react-intl.

use murmur3::murmur3_32;
use std::io::Cursor;

/// Generates a murmur3 hash with seed=0 to match babel-plugin-react-intl behavior.
///
/// # Arguments
/// * `input` - The input string to hash
///
/// # Returns
/// The hash value as a decimal string
///
/// # Example
/// ```
/// use react_intl_core::id_generator::murmur32_hash;
/// let hash = murmur32_hash("hello world");
/// assert_eq!(hash, "1586663183");
/// ```
pub fn murmur32_hash(input: &str) -> String {
    let mut cursor = Cursor::new(input.as_bytes());
    murmur3_32(&mut cursor, 0).unwrap_or(0).to_string()
}

/// Generates a base64 encoded string.
///
/// # Arguments
/// * `message` - The input string to hash
///
/// # Returns
/// The encoded value as a string
///
/// # Example
/// ```
/// use react_intl_core::id_generator::base64_hash;
/// let hash = base64_hash("hello world");
/// assert_eq!(hash, "aGVsbG8gd29ybGQ=");
/// ```
pub fn base64_hash(input: &str) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    STANDARD.encode(input.as_bytes())
}

/// Hashes a string using the specified algorithm.
///
/// # Arguments
/// * `input` - The input string to hash
/// * `algorithm` - The hash algorithm: "murmur3" or "base64"
///
/// # Returns
/// The hashed string
///
/// # Example
/// ```
/// use react_intl_core::id_generator::hash_string;
/// let hash = hash_string("hello", "murmur3");
/// ```
pub fn hash_string(input: &str, algorithm: &str) -> String {
    match algorithm {
        "base64" => base64_hash(input),
        "murmur3" | _ => murmur32_hash(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murmur32_hash() {
        let hash1 = murmur32_hash("test message");
        let hash2 = murmur32_hash("test message");
        assert_eq!(hash1, hash2, "Same input should produce same hash");

        let hash3 = murmur32_hash("different message");
        assert_ne!(
            hash1, hash3,
            "Different inputs should produce different hashes"
        );
    }

    #[test]
    fn test_hash_string_murmur3() {
        let result = hash_string("hello", "murmur3");
        assert_eq!(result, murmur32_hash("hello"));
    }

    #[test]
    fn test_base64_hash() {
        let result = hash_string("hello", "base64");
        assert_eq!(result, "aGVsbG8=");
    }

    #[test]
    fn test_hash_string_default() {
        // Unknown algorithm defaults to murmur3
        let result = hash_string("hello", "unknown");
        assert_eq!(result, murmur32_hash("hello"));
    }
}
