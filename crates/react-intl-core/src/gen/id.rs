//! ID generation utilities
//!
//! Provides hash functions for generating message IDs compatible with babel-plugin-react-intl.

use murmur3::murmur3_32;
use std::io::Cursor;

use crate::gen::path::add_prefix;
use crate::types::CoreState;

/// Generates a murmur3 hash with seed=0 and encodes it as base64.
/// Matches the behavior of the JS implementation.
///
/// # Arguments
/// * `input` - The input string to hash
///
/// # Returns
/// The hash value as a base64 encoded string
fn murmur32_hash(input: &str) -> String {
    let mut cursor = Cursor::new(input.as_bytes());
    let hash = murmur3_32(&mut cursor, 0).unwrap_or(0);
    // Convert u32 to big-endian bytes and encode as base64
    let bytes = hash.to_be_bytes();
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    STANDARD.encode(bytes)
}

/// Hashes a string using murmur3 algorithm.
///
/// # Arguments
/// * `input` - The input string to hash
///
/// # Returns
/// The hashed string as base64 encoded murmur3 hash
pub fn hash_string(input: &str, _algorithm: &str) -> String {
    // Always use murmur3, algorithm parameter kept for API extensibility
    murmur32_hash(input)
}

/// payload to generate id from message description
/// object
/// ```js
/// const messages = intl.formatMessage({
///     defaultMessage: "defaultMessage", // hash default message
///     description: "description", // + hash description
/// });
/// ```
/// jsx
/// ```js
/// <FormattedMessage defaultMessage="hello" /> // hash default message
/// ```
#[derive(Debug, Clone)]
pub struct GenIdFromDescriptorPayload<'a> {
    pub default_message: &'a String,
    pub description: &'a Option<String>,
}

/// payload to generate id from key + part of message description
/// only key
/// ```js
/// const messages = defineMessages({
///     hello: { // key = "hello"
///         defaultMessage: "defaultMessage",
///         description: "description", // + hash description
///     }
/// });
/// ```
/// key + descriptor path
/// ```js
/// const messages = defineMessages({
///     hello: { // key = "hello"
///         defaultMessage: "defaultMessage",
///         description: "description", // + hash description
///     }
/// });
/// ```
#[derive(Debug, Clone)]
pub struct GenIdFromKeyPayload<'a> {
    pub key: &'a String,
    pub description: &'a Option<String>,
}

#[derive(Debug, Clone)]
pub enum GenIdPayload<'a> {
    Key(GenIdFromKeyPayload<'a>),
    Descriptor(GenIdFromDescriptorPayload<'a>),
}

/// Generates an ID for a message based on the configuration
///
/// # Arguments
/// * `state` - The core state
/// * `payload` - Payload for id generation
pub fn generate_message_id(state: &CoreState, payload: &GenIdPayload) -> String {
    let raw_id = match payload {
        GenIdPayload::Key(key_payload) => {
            let mut parts = vec![key_payload.key.to_owned()];
            if let Some(description) = &key_payload.description {
                parts.push(murmur32_hash(description.as_str()));
            }
            parts.join(state.opts.separator.as_str())
        }
        GenIdPayload::Descriptor(descriptor) => {
            let mut parts = vec![descriptor.default_message.to_owned()];
            if let Some(description) = &descriptor.description {
                parts.push(description.to_owned());
            }
            murmur32_hash(parts.join(state.opts.separator.as_str()).as_str())
        }
    };

    let path_id = add_prefix(state, &raw_id);

    // Apply hash_id option if enabled
    if state.opts.hash_id {
        hash_string(&path_id, &state.opts.hash_algorithm)
    } else {
        path_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murmur32_hash() {
        // Test with known values (base64 encoded murmur3 hash)
        let hello_hash = murmur32_hash("hello");
        let test_msg_hash = murmur32_hash("test message");
        let default_msg_hash = murmur32_hash("defaultMessage");

        // Verify hashes are valid base64 strings
        assert!(!hello_hash.is_empty());
        assert!(!test_msg_hash.is_empty());
        assert!(!default_msg_hash.is_empty());

        // Verify exact values
        assert_eq!(hello_hash, "JIv6Rw==");

        // Test consistency
        let hash1 = murmur32_hash("test message");
        let hash2 = murmur32_hash("test message");
        assert_eq!(hash1, hash2, "Same input should produce same hash");

        // Test different inputs produce different hashes
        let hash3 = murmur32_hash("different message");
        assert_ne!(
            hash1, hash3,
            "Different inputs should produce different hashes"
        );
    }

    #[test]
    fn test_hash_string() {
        let result = hash_string("hello", "murmur3");
        assert_eq!(result, murmur32_hash("hello"));

        // Algorithm parameter is now ignored, should still return murmur3 hash
        let result2 = hash_string("hello", "ignored");
        assert_eq!(result2, murmur32_hash("hello"));
    }
}
