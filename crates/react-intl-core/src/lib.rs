//! Core library for React Intl message extraction and ID generation
//! Used by both SWC plugin and CLI tool

pub mod id_generator;
pub mod message_extractor;
pub mod path_utils;
pub mod types;

// Re-export main types and functions for convenience
pub use types::*;
