//! Core library for React Intl message extraction and ID generation
//!
//! This library provides shared functionality for the SWC plugin and CLI tool,
//! ensuring consistent ID generation and message extraction across both components.

pub mod ast_analysis;
pub mod id_generator;
pub mod import_check;
pub mod path_utils;
pub mod types;

// Re-export main types and functions for convenience
pub use types::{CoreOptions, CoreState, OutputMode, RemovePrefix, REACT_COMPONENTS};

pub use import_check::process_import_decl;

pub use ast_analysis::{
    analyze_define_messages, analyze_format_message, analyze_jsx_element, extract_prop_name,
    TransformedMessageData,
};
