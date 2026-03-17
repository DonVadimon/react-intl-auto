//! napi-rs bindings for React Intl Extract CLI
//!
//! This module provides JavaScript bindings for the CLI functionality
//! using napi-rs to enable native Node.js module support.

use clap::Parser;
use napi_derive::napi;
use react_intl_core::types::{CoreOptions, OutputMode, RemovePrefix};

// Re-export core module for use in both binary and JS bindings
pub mod core;
pub mod extractor;
mod visitors;

use crate::core::{run_cli as core_run_cli, Args, extract_from_file, find_files};

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

impl From<extractor::ExtractedMessage> for JsExtractedMessage {
    fn from(msg: extractor::ExtractedMessage) -> Self {
        Self {
            id: msg.id,
            default_message: msg.default_message.unwrap_or_default(),
            description: msg.description,
            file: msg.file,
        }
    }
}

/// Extract result structure for JS
#[napi(object)]
pub struct JsExtractResult {
    pub messages: Vec<JsExtractedMessage>,
    pub files_processed: u32,
}

/// Run CLI with arguments from JS
/// Returns exit code (0 for success, 1 for error)
#[napi]
pub fn run_cli(args: Vec<String>) -> i32 {
    // Parse Args from string vector
    let args = match parse_args_from_vec(args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return 1;
        }
    };

    match core_run_cli(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}

/// Extract messages from files matching glob patterns (sync)
#[napi]
pub fn extract_sync(
    patterns: Vec<String>,
    options: Option<JsExtractOptions>,
) -> napi::Result<JsExtractResult> {
    use std::path::PathBuf;
    
    let opts = options.map(|o| o.to_core_options()).unwrap_or_default();
    
    // Convert string patterns to PathBuf
    let path_patterns: Vec<PathBuf> = patterns.iter().map(|p| PathBuf::from(p)).collect();
    
    // Default ignore patterns
    let ignore_patterns: Vec<PathBuf> = vec![
        PathBuf::from("**/node_modules/**"),
        PathBuf::from("**/.git/**"),
    ];
    
    // Find files matching patterns
    let files = find_files(&path_patterns, &ignore_patterns)
        .map_err(|e| napi::Error::from_reason(format!("Failed to find files: {}", e)))?;
    
    let mut messages = Vec::new();
    let files_processed = files.len() as u32;
    
    for file in files {
        match extract_from_file(&file, &opts) {
            Ok(msgs) => {
                for msg in msgs {
                    messages.push(msg.into());
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to process {}: {}", file.display(), e);
            }
        }
    }
    
    Ok(JsExtractResult {
        messages,
        files_processed,
    })
}

/// Extract messages from files matching glob patterns (async)
#[napi]
pub async fn extract(
    patterns: Vec<String>,
    options: Option<JsExtractOptions>,
) -> napi::Result<JsExtractResult> {
    // For now, just call sync version
    // In the future, this could be parallelized with tokio
    extract_sync(patterns, options)
}

/// Parse a single file and extract messages
#[napi]
pub fn parse_file(
    file_path: String,
    options: Option<JsExtractOptions>,
) -> napi::Result<Vec<JsExtractedMessage>> {
    use std::path::PathBuf;
    
    let opts = options.map(|o| o.to_core_options()).unwrap_or_default();
    let path = PathBuf::from(&file_path);
    
    let messages = extract_from_file(&path, &opts)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse file: {}", e)))?;
    
    Ok(messages.into_iter().map(|m| m.into()).collect())
}

/// Parse arguments from a vector of strings
/// Uses clap's parse_from to properly parse CLI arguments
fn parse_args_from_vec(args: Vec<String>) -> anyhow::Result<Args> {
    // Use clap's parse_from which handles all the argument parsing
    // The first argument is typically the program name, so we keep it
    let args = Args::parse_from(args);
    Ok(args)
}
