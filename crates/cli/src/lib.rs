//! napi-rs bindings for React Intl Extract CLI
//!
//! This module provides JavaScript bindings for the CLI functionality
//! using napi-rs to enable native Node.js module support.

use napi_derive::napi;
use react_intl_core::types::{CoreOptions, OutputMode, RemovePrefix};

/// JavaScript-compatible options for extraction
#[napi(object)]
pub struct JsExtractOptions {
    pub remove_prefix: Option<String>,
    pub module_source_name: Option<String>,
    pub separator: Option<String>,
    pub relative_to: Option<String>,
    pub hash_id: Option<bool>,
    pub hash_algorithm: Option<String>,
    pub extract_source_location: Option<bool>,
    pub output_mode: Option<String>,
}

impl JsExtractOptions {
    /// Convert JS options to CoreOptions
    fn to_core_options(&self) -> CoreOptions {
        let remove_prefix = self.remove_prefix.as_ref().and_then(|s| match s.as_str() {
            "true" => Some(RemovePrefix::Boolean(true)),
            "false" => Some(RemovePrefix::Boolean(false)),
            "" => None,
            _ => Some(RemovePrefix::String(s.clone())),
        });

        let output_mode = self
            .output_mode
            .as_ref()
            .map(|s| match s.as_str() {
                "perfile" => OutputMode::PerFile,
                _ => OutputMode::Aggregated,
            })
            .unwrap_or(OutputMode::Aggregated);

        CoreOptions {
            remove_prefix,
            module_source_name: self
                .module_source_name
                .clone()
                .unwrap_or_else(|| "react-intl".to_string()),
            separator: self.separator.clone().unwrap_or_else(|| ".".to_string()),
            relative_to: self.relative_to.clone(),
            hash_id: self.hash_id.unwrap_or(false),
            hash_algorithm: self
                .hash_algorithm
                .clone()
                .unwrap_or_else(|| "murmur3".to_string()),
            extract_source_location: self.extract_source_location.unwrap_or(false),
            output_mode,
        }
    }
}

/// Extracted message structure for JS
#[napi(object)]
pub struct JsExtractedMessage {
    pub id: String,
    pub default_message: String,
    pub description: Option<String>,
    pub file: Option<String>,
}

/// Extract result structure for JS
#[napi(object)]
pub struct JsExtractResult {
    pub messages: Vec<JsExtractedMessage>,
    pub files_processed: u32,
}

/// Run CLI with arguments
/// Returns exit code (0 for success, 1 for error)
#[napi]
pub fn run_cli(args: Vec<String>) -> i32 {
    // TODO: Implement in HYBRID_EXTRACT-009-002
    // This will call the main CLI logic from main.rs
    0
}

/// Extract messages from files matching glob patterns (async)
#[napi]
pub async fn extract(
    patterns: Vec<String>,
    options: Option<JsExtractOptions>,
) -> napi::Result<JsExtractResult> {
    // TODO: Implement in HYBRID_EXTRACT-009-001
    // This will use the extractor module from main.rs
    Ok(JsExtractResult {
        messages: vec![],
        files_processed: 0,
    })
}

/// Extract messages from files matching glob patterns (sync)
#[napi]
pub fn extract_sync(
    patterns: Vec<String>,
    options: Option<JsExtractOptions>,
) -> napi::Result<JsExtractResult> {
    // TODO: Implement in HYBRID_EXTRACT-009-001
    Ok(JsExtractResult {
        messages: vec![],
        files_processed: 0,
    })
}

/// Parse a single file and extract messages
#[napi]
pub fn parse_file(
    file_path: String,
    options: Option<JsExtractOptions>,
) -> napi::Result<Vec<JsExtractedMessage>> {
    // TODO: Implement in HYBRID_EXTRACT-009-001
    Ok(vec![])
}
