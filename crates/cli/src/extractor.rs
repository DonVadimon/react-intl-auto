//! Message extraction from AST
//!
//! This module provides functionality to extract React Intl messages from source code.
//! It uses read-only visitors for CLI message extraction.

use pathdiff::diff_paths;
use react_intl_core::types::{CoreOptions, CoreState, TransformedMessageData};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use swc_core::ecma::ast::*;
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::visitors::call::CallExpressionVisitor;
use crate::visitors::import::ImportVisitor;
use crate::visitors::jsx::JSXVisitor;
use crate::visitors::vars::VarVisitor;

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

fn detect_syntax(filename: &PathBuf) -> Syntax {
    let ext = filename.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "tsx" => Syntax::Typescript(swc_core::ecma::parser::TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        "ts" | "mts" => Syntax::Typescript(swc_core::ecma::parser::TsSyntax {
            tsx: false,
            ..Default::default()
        }),
        _ => Syntax::Es(swc_core::ecma::parser::EsSyntax {
            jsx: true,
            ..Default::default()
        }),
    }
}

/// Extracts messages from source code
///
/// # Arguments
/// * `code` - The source code to analyze
/// * `filename` - The filename (used for ID generation)
/// * `options` - Core options for extraction
///
/// # Returns
/// Result with vector of extracted messages or error string
pub fn extract_messages(
    code: &str,
    filename: &PathBuf,
    options: &CoreOptions,
) -> Result<Vec<ExtractedMessage>, String> {
    let syntax = detect_syntax(filename);

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
            return Err(format!(
                "Failed to parse {}: {:#?}",
                filename.to_string_lossy(),
                err
            ));
        }
    };

    // Create visitor and extract messages
    let mut visitor = MessageExtractorVisitor::new(PathBuf::from(filename), options.clone());

    module.visit_with(&mut visitor);

    visitor.into_result()
}

/// Visitor for extracting messages from AST
///
/// This visitor can be used with already parsed AST
pub struct MessageExtractorVisitor {
    state: CoreState,
    filename: PathBuf,
    messages: Vec<TransformedMessageData>,
    errors: Vec<String>,
}

impl MessageExtractorVisitor {
    pub fn new(filename: PathBuf, options: CoreOptions) -> Self {
        let state = CoreState::new(filename.clone(), options);

        Self {
            state,
            filename,
            messages: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn into_result(self) -> Result<Vec<ExtractedMessage>, String> {
        if !self.errors.is_empty() {
            return Err(self.errors.join("\n"));
        }

        Ok(self
            .messages
            .into_iter()
            .map(|transformed| {
                to_extracted_message(
                    &transformed,
                    &self.filename,
                    self.state.opts.extract_source_location,
                )
            })
            .collect())
    }
}

impl Visit for MessageExtractorVisitor {
    fn visit_module(&mut self, module: &Module) {
        // First pass: collect imported names and aliases
        let mut import_visitor = ImportVisitor::new(&self.state);
        let mut var_visitor = VarVisitor::new(&self.state);
        module.visit_with(&mut import_visitor);
        module.visit_with(&mut var_visitor);

        let mut jsx_visitor = JSXVisitor::new(&self.state, &import_visitor);
        let mut call_visitor =
            CallExpressionVisitor::new(&self.state, &import_visitor, &var_visitor);

        module.visit_with(&mut jsx_visitor);
        module.visit_with(&mut call_visitor);

        let (jsx_messages, jsx_errors) = jsx_visitor.into_result();
        let (call_messages, call_errors) = call_visitor.into_result();

        self.messages.extend(jsx_messages);
        self.messages.extend(call_messages);
        self.errors.extend(jsx_errors);
        self.errors.extend(call_errors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_syntax_tsx() {
        let syntax = detect_syntax(&PathBuf::from("src/components/App.tsx"));
        match syntax {
            Syntax::Typescript(ts) => assert!(ts.tsx),
            _ => panic!("Expected TypeScript syntax for .tsx file"),
        }
    }

    #[test]
    fn test_detect_syntax_ts() {
        let syntax = detect_syntax(&PathBuf::from("src/components/utils.ts"));
        match syntax {
            Syntax::Typescript(ts) => assert!(!ts.tsx),
            _ => panic!("Expected TypeScript syntax for .ts file"),
        }
    }

    #[test]
    fn test_detect_syntax_mts() {
        let syntax = detect_syntax(&PathBuf::from("src/components/utils.mts"));
        match syntax {
            Syntax::Typescript(ts) => assert!(!ts.tsx),
            _ => panic!("Expected TypeScript syntax for .mts file"),
        }
    }

    #[test]
    fn test_detect_syntax_jsx() {
        let syntax = detect_syntax(&PathBuf::from("src/components/App.jsx"));
        match syntax {
            Syntax::Es(es) => assert!(es.jsx),
            _ => panic!("Expected ES syntax for .jsx file"),
        }
    }

    #[test]
    fn test_detect_syntax_js() {
        let syntax = detect_syntax(&PathBuf::from("src/components/utils.js"));
        match syntax {
            Syntax::Es(es) => assert!(es.jsx),
            _ => panic!("Expected ES syntax for .js file"),
        }
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
