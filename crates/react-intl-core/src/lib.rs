//! Core library for React Intl message extraction and ID generation
//!
//! This library provides shared functionality for the SWC plugin and CLI tool,
//! ensuring consistent ID generation and message extraction across both components.

pub mod id_generator;
pub mod message_extractor;
pub mod path_utils;
pub mod types;

// Re-export main types and functions for convenience
pub use types::{CoreOptions, CoreState, IncludeExportName, RemovePrefix};

pub use id_generator::{murmur32_hash, hash_string};

pub use path_utils::{dot_path, dot_path_replace, find_project_root, get_prefix};

pub use message_extractor::{extract_messages, ExtractedMessage, ExtractionOptions};
