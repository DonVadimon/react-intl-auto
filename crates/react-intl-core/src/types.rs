//! Core types for React Intl message extraction
//!
//! This module will be implemented in HYBRID_EXTRACT-002

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Options for the plugin/core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreOptions {
    #[serde(default, alias = "removePrefix")]
    pub remove_prefix: Option<RemovePrefix>,
    #[serde(default)]
    pub filebase: bool,
    #[serde(default, alias = "includeExportName")]
    pub include_export_name: Option<IncludeExportName>,
    #[serde(default, alias = "extractComments")]
    pub extract_comments: bool,
    #[serde(default, alias = "useKey")]
    pub use_key: bool,
    #[serde(default = "default_module_source_name", alias = "moduleSourceName")]
    pub module_source_name: String,
    #[serde(default = "default_separator")]
    pub separator: String,
    #[serde(default, alias = "relativeTo")]
    pub relative_to: Option<String>,
    #[serde(default, alias = "hashId")]
    pub hash_id: bool,
    #[serde(default = "default_hash_algorithm", alias = "hashAlgorithm")]
    pub hash_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemovePrefix {
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncludeExportName {
    Boolean(bool),
    All,
}

impl Default for CoreOptions {
    fn default() -> Self {
        Self {
            remove_prefix: None,
            filebase: false,
            include_export_name: None,
            extract_comments: true,
            use_key: false,
            module_source_name: "react-intl".to_string(),
            separator: ".".to_string(),
            relative_to: None,
            hash_id: false,
            hash_algorithm: "murmur3".to_string(),
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
