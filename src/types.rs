use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginOptions {
    #[serde(default, alias = "removePrefix")]
    pub remove_prefix: Option<RemovePrefix>,
    #[serde(default)]
    pub filebase: bool,
    #[serde(default, alias = "includeExportName")]
    pub include_export_name: Option<IncludeExportName>,
    #[serde(default)]
    pub extract_comments: bool,
    #[serde(default)]
    pub use_key: bool,
    #[serde(default = "default_module_source_name")]
    pub module_source_name: String,
    #[serde(default = "default_separator")]
    pub separator: String,
    #[serde(default)]
    pub relative_to: Option<String>,
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

impl Default for PluginOptions {
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
        }
    }
}

fn default_module_source_name() -> String {
    "react-intl".to_string()
}

fn default_separator() -> String {
    ".".to_string()
}

#[derive(Debug, Clone)]
pub struct PluginState {
    pub filename: PathBuf,
    pub opts: PluginOptions,
}

impl PluginState {
    pub fn new(filename: PathBuf, opts: PluginOptions) -> Self {
        Self { filename, opts }
    }
}
