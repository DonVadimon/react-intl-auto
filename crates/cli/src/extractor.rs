//! Message extraction from AST
//!
//! This module provides functionality to extract React Intl messages from source code.
//! It uses read-only visitors for CLI message extraction.

use pathdiff::diff_paths;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use swc_core::ecma::ast::*;
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_core::ecma::visit::{Visit, VisitWith};

use react_intl_core::*;

use crate::visitors::call::CallExpressionVisitor;
use crate::visitors::import::ImportVisitor;
use crate::visitors::jsx::JSXVisitor;

/// Extracted message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedMessage {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

fn relative_to_cwd(absolute_path: &PathBuf) -> PathBuf {
    let cwd = std::env::current_dir().expect("Failed to get current working directory");

    diff_paths(absolute_path, cwd).unwrap_or(absolute_path.clone())
}

/// Converts TransformedMessageData to ExtractedMessage
fn to_extracted_message(
    transformed: &TransformedMessageData,
    filename: &PathBuf,
    include_source_location: bool,
) -> ExtractedMessage {
    ExtractedMessage {
        id: transformed.id.clone(),
        default_message: transformed.default_message.clone(),
        description: transformed.description.clone(),
        file: if include_source_location {
            Some(relative_to_cwd(filename).to_string_lossy().to_string())
        } else {
            None
        },
    }
}

/// Detects typescript syntsx in file by extension
fn is_ts_file(filename: &PathBuf) -> bool {
    ["ts", "mts", "tsx"]
        .iter()
        .any(|ext| match filename.extension() {
            Some(file_ext) => file_ext.to_os_string().into_string().unwrap() == *ext,
            None => false,
        })
}

/// Extracts messages from source code
///
/// # Arguments
/// * `code` - The source code to analyze
/// * `filename` - The filename (used for ID generation)
/// * `options` - Core options for extraction
///
/// # Returns
/// Vector of extracted messages
///
/// # Example
/// ```
/// use react_intl_core::{extract_messages, CoreOptions};
///
/// let code = r#"
/// import { defineMessages } from 'react-intl';
/// export default defineMessages({
///   hello: 'Hello World'
/// });
/// "#;
///
/// let options = CoreOptions::default();
/// let messages = extract_messages(code, "test.js", &options);
/// ```
pub fn extract_messages(
    code: &str,
    filename: &PathBuf,
    options: &CoreOptions,
) -> Vec<ExtractedMessage> {
    let syntax = if is_ts_file(filename) {
        Syntax::Typescript(swc_core::ecma::parser::TsSyntax {
            tsx: true,
            ..Default::default()
        })
    } else {
        Syntax::Es(swc_core::ecma::parser::EsSyntax {
            jsx: true,
            ..Default::default()
        })
    };

    // Create lexer and parser
    let input = StringInput::new(
        code,
        swc_core::common::BytePos(0),
        swc_core::common::BytePos(code.len() as u32),
    );
    let lexer = Lexer::new(syntax, EsVersion::Es2024, input, None);
    let mut parser = Parser::new_from(lexer);

    // Parse the source code
    let module = match parser.parse_module() {
        Ok(module) => module,
        Err(err) => {
            eprintln!("Failed to parse {}: {:#?}", filename.to_string_lossy(), err);
            return vec![];
        }
    };

    // Create visitor and extract messages
    let mut visitor = MessageExtractorVisitor::new(PathBuf::from(filename), options.clone());

    module.visit_with(&mut visitor);

    visitor.into_messages()
}

/// Visitor for extracting messages from AST
///
/// This visitor can be used with already parsed AST
pub struct MessageExtractorVisitor {
    state: CoreState,
    filename: PathBuf,
    messages: Vec<TransformedMessageData>,
}

impl MessageExtractorVisitor {
    pub fn new(filename: PathBuf, options: CoreOptions) -> Self {
        let state = CoreState::new(filename.clone(), options);

        Self {
            state,
            filename,
            messages: Vec::new(),
        }
    }

    pub fn into_messages(self) -> Vec<ExtractedMessage> {
        self.messages
            .into_iter()
            .map(|transformed| {
                to_extracted_message(
                    &transformed,
                    &self.filename,
                    self.state.opts.extract_source_location,
                )
            })
            .collect()
    }
}

impl Visit for MessageExtractorVisitor {
    fn visit_module(&mut self, module: &Module) {
        // First pass: collect imported names and aliases
        let mut import_visitor = ImportVisitor::new(&self.state);
        module.visit_with(&mut import_visitor);

        let mut jsx_visitor = JSXVisitor::new(&self.state, &import_visitor);
        let mut call_visitor = CallExpressionVisitor::new(&self.state, &import_visitor);

        module.visit_with(&mut jsx_visitor);
        module.visit_with(&mut call_visitor);

        self.messages.append(&mut jsx_visitor.messages);
        self.messages.append(&mut call_visitor.messages);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ts_file() {
        assert!(is_ts_file(&PathBuf::from("src/components/App.tsx")));
        assert!(is_ts_file(&PathBuf::from("src/components/utils.ts")));
        assert!(is_ts_file(&PathBuf::from("src/components/utils.mts")));
        assert!(!is_ts_file(&PathBuf::from("src/components/utils.js")));
        assert!(!is_ts_file(&PathBuf::from("src/components/App.css")));
        assert!(!is_ts_file(&PathBuf::from(".eslintrc")));
    }

    #[test]
    fn test_to_extracted_message() {
        let transformed = TransformedMessageData {
            id: "test.hello".to_string(),
            default_message: Some("Hello World".to_string()),
            description: Some("A greeting".to_string()),
        };
        let filename = PathBuf::from("test.js");

        let extracted = to_extracted_message(&transformed, &filename, false);

        assert_eq!(extracted.id, "test.hello");
        assert_eq!(extracted.default_message, Some("Hello World".to_string()));
        assert_eq!(extracted.description, Some("A greeting".to_string()));
        assert!(extracted.file.is_none()); // Not included by default
    }

    #[test]
    fn test_to_extracted_message_with_source_location() {
        let transformed = TransformedMessageData {
            id: "test.hello".to_string(),
            default_message: Some("Hello".to_string()),
            description: None,
        };
        let filename = PathBuf::from("test.js");

        let extracted = to_extracted_message(&transformed, &filename, true);

        assert!(extracted.file.is_some());
    }
}
