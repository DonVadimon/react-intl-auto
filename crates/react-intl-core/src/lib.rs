//! Core library for React Intl message extraction and ID generation
//!
//! This library provides shared functionality for the SWC plugin and CLI tool,
//! ensuring consistent ID generation and message extraction across both components.

pub mod ast_analysis;
pub mod id_generator;
pub mod message_extractor;
pub mod path_utils;
pub mod types;

// Re-export main types and functions for convenience
pub use types::{CoreOptions, CoreState, IncludeExportName, RemovePrefix};

pub use id_generator::{hash_string, murmur32_hash};

pub use path_utils::{dot_path, dot_path_replace, find_project_root, get_prefix};

pub use message_extractor::{extract_messages, ExtractedMessage, ExtractionOptions};

pub use ast_analysis::{
    analyze_define_messages, analyze_format_message, analyze_jsx_element, generate_message_id,
    MessageData, TransformedMessageData,
};
