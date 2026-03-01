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
    transformed: TransformedMessageData,
    filename: &PathBuf,
    include_source_location: bool,
) -> ExtractedMessage {
    ExtractedMessage {
        id: transformed.id,
        default_message: transformed.default_message,
        description: transformed.description,
        file: if include_source_location {
            Some(relative_to_cwd(filename).to_string_lossy().to_string())
        } else {
            None
        },
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
    // Determine syntax based on file extension
    let is_ts = [".ts", ".mts", ".tsx"]
        .iter()
        .any(|ext| filename.ends_with(ext));
    let is_tsx = filename.ends_with(".tsx");
    // Enable JSX for both .jsx and .js files (many projects use JSX in .js files)
    let is_jsx = filename.ends_with(".jsx") || filename.ends_with(".js");

    let syntax = if is_ts {
        Syntax::Typescript(swc_core::ecma::parser::TsSyntax {
            tsx: is_tsx,
            ..Default::default()
        })
    } else {
        Syntax::Es(swc_core::ecma::parser::EsSyntax {
            jsx: is_jsx,
            ..Default::default()
        })
    };

    // Create lexer and parser
    let input = StringInput::new(
        code,
        swc_core::common::BytePos(0),
        swc_core::common::BytePos(code.len() as u32),
    );
    let lexer = Lexer::new(syntax, Default::default(), input, None);
    let mut parser = Parser::new_from(lexer);

    // Parse the source code
    let module = match parser.parse_module() {
        Ok(module) => module,
        Err(err) => {
            eprintln!("Failed to parse {}: {:?}", filename.to_string_lossy(), err);
            return vec![];
        }
    };

    // Create visitor and extract messages
    let mut visitor = MessageExtractorVisitor::new(
        // relative_filepath,
        PathBuf::from(filename),
        options.clone(),
    );
    module.visit_with(&mut visitor);

    visitor.into_messages()
}

/// Visitor for extracting messages from AST
///
/// This visitor can be used with already parsed AST
pub struct MessageExtractorVisitor {
    messages: Vec<ExtractedMessage>,
    filename: PathBuf,
    imported_names: std::collections::HashSet<String>,
    alias_map: std::collections::HashMap<String, String>,
    state: CoreState,
    define_messages_counter: usize,
}

impl MessageExtractorVisitor {
    pub fn new(filename: PathBuf, options: CoreOptions) -> Self {
        let state = CoreState::new(filename.clone(), options);

        Self {
            messages: Vec::new(),
            filename,
            imported_names: std::collections::HashSet::new(),
            alias_map: std::collections::HashMap::new(),
            state,
            define_messages_counter: 0,
        }
    }

    pub fn into_messages(self) -> Vec<ExtractedMessage> {
        self.messages
    }

    fn add_message(&mut self, transformed: TransformedMessageData) {
        let extracted = to_extracted_message(
            transformed,
            &self.filename,
            self.state.opts.extract_source_location,
        );
        self.messages.push(extracted);
    }
}

impl Visit for MessageExtractorVisitor {
    fn visit_import_decl(&mut self, import: &ImportDecl) {
        let (imported_names, alias_map) = process_import_decl(import, &self.state);

        self.imported_names = imported_names;
        self.alias_map = alias_map;
    }

    fn visit_jsx_element(&mut self, element: &JSXElement) {
        element.visit_children_with(self);

        if let JSXElementName::Ident(name) = &element.opening.name {
            let name_str = name.sym.as_str().to_string();
            let component_name = self.alias_map.get(&name_str).unwrap_or(&name_str);

            // Check if this is a React Intl component and it was imported
            if REACT_COMPONENTS.contains(&component_name.as_str())
                && self.imported_names.contains(&name_str)
            {
                if let Some((transformed, _)) = analyze_jsx_element(element, &self.state) {
                    self.add_message(transformed);
                }
            }
        }
    }

    fn visit_call_expr(&mut self, call: &CallExpr) {
        call.visit_children_with(self);

        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                let fn_name = ident.sym.to_string();

                // Check if this function was imported
                if self.imported_names.contains(&fn_name) {
                    // Check if it's defineMessages directly or via alias
                    let original_name = self
                        .alias_map
                        .get(&fn_name)
                        .map(|s| s.as_str())
                        .unwrap_or(fn_name.as_str());

                    if original_name == "defineMessages" {
                        let results = analyze_define_messages(call, &self.state);
                        if !results.is_empty() {
                            for (_, transformed, __) in results {
                                self.add_message(transformed);
                            }
                            self.define_messages_counter += 1;
                        }
                    } else if original_name == "formatMessage" {
                        if let Some((transformed, _)) = analyze_format_message(call, &self.state) {
                            self.add_message(transformed);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_extracted_message() {
        let transformed = TransformedMessageData {
            id: "test.hello".to_string(),
            default_message: Some("Hello World".to_string()),
            description: Some("A greeting".to_string()),
        };
        let filename = PathBuf::from("test.js");

        let extracted = to_extracted_message(transformed, &filename, false);

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

        let extracted = to_extracted_message(transformed, &filename, true);

        assert!(extracted.file.is_some());
    }
}
