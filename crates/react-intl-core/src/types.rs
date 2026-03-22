//! Core types for React Intl message extraction

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Options for the plugin/core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreOptions {
    /// Remove prefix from file path
    #[serde(default, alias = "removePrefix")]
    pub remove_prefix: Option<RemovePrefix>,
    /// react-intl module source name
    #[serde(default = "default_module_source_name", alias = "moduleSourceName")]
    pub module_source_name: String,
    /// ID separator
    #[serde(default = "default_separator")]
    pub separator: String,
    /// Relative path for ID generation
    #[serde(default, alias = "relativeTo")]
    pub relative_to: Option<String>,
    /// Apply hash fn to id
    #[serde(default, alias = "hashId")]
    pub hash_id: bool,
    /// Hash fn for id
    #[serde(default = "default_hash_algorithm", alias = "hashAlgorithm")]
    pub hash_algorithm: String,
    /// ### CLI only option
    /// Add "file" field to extracted messages data
    #[serde(default, alias = "extractSourceLocation")]
    pub extract_source_location: bool,
    /// ### CLI only option
    /// Output mode: "aggregated" (single file) or "perfile" (separate files per source)
    #[serde(default, alias = "outputMode")]
    pub output_mode: OutputMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemovePrefix {
    Boolean(bool),
    String(String),
}

/// Output mode for CLI extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum OutputMode {
    /// Single aggregated JSON file
    #[default]
    Aggregated,
    /// Separate JSON files per source file
    PerFile,
}


impl Default for CoreOptions {
    fn default() -> Self {
        Self {
            remove_prefix: None,
            module_source_name: "react-intl".to_string(),
            separator: ".".to_string(),
            relative_to: None,
            hash_id: false,
            hash_algorithm: "murmur3".to_string(),
            extract_source_location: false,
            output_mode: OutputMode::Aggregated,
        }
    }
}

fn default_module_source_name() -> String {
    "react-intl".to_string()
}

fn default_separator() -> String {
    ".".to_string()
}

fn default_hash_algorithm() -> String {
    "murmur3".to_string()
}

/// Plugin state
#[derive(Debug, Clone)]
pub struct CoreState {
    pub filename: PathBuf,
    pub opts: CoreOptions,
}

impl CoreState {
    pub fn new(filename: PathBuf, opts: CoreOptions) -> Self {
        Self { filename, opts }
    }
}

pub const REACT_COMPONENTS: &[&str] = &["FormattedMessage", "FormattedHTMLMessage"];

/// Data structure representing a transformed message with generated ID
#[derive(Debug, Clone)]
pub struct TransformedMessageData {
    pub id: String,
    pub default_message: Option<String>,
    pub description: Option<String>,
}
