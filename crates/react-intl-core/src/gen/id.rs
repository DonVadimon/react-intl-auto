//! ID generation utilities
//!
//! Provides hash functions for generating message IDs compatible with babel-plugin-react-intl.

use murmur3::murmur3_32;
use std::io::Cursor;

use crate::gen::path::add_prefix;
use crate::types::CoreState;

/// Generates a murmur3 hash with seed=0 to match babel-plugin-react-intl behavior.
///
/// # Arguments
/// * `input` - The input string to hash
///
/// # Returns
/// The hash value as a decimal string
fn murmur32_hash(input: &str) -> String {
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
fn base64_hash(input: &str) -> String {
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
pub fn hash_string(input: &str, algorithm: &str) -> String {
    match algorithm {
        "base64" => base64_hash(input),
        "murmur3" | _ => murmur32_hash(input),
    }
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
    fn test_base64_hash() {
        let hash1 = base64_hash("hello");
        let hash2 = base64_hash("hello");
        assert_eq!(hash1, hash2, "Same input should produce same hash");

        let hash3 = base64_hash("different message");
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
    fn test_hash_string_base64() {
        let result = hash_string("hello", "base64");
        assert_eq!(result, base64_hash("hello"));
    }

    #[test]
    fn test_hash_string_default() {
        // Unknown algorithm defaults to murmur3
        let result = hash_string("hello", "unknown");
        assert_eq!(result, murmur32_hash("hello"));
    }
}
