//! Message extraction from AST
//!
//! This module provides functionality to extract React Intl messages from source code.
//! It uses read-only visitors for CLI message extraction.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::ast_analysis::{
    analyze_define_messages, analyze_format_message, analyze_jsx_element, MessageData,
    TransformedMessageData,
};
use crate::types::{CoreOptions, CoreState};

/// Extracted message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedMessage {
    pub id: String,
    pub default_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
}

/// Options for message extraction
#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub hash_id: bool,
    pub hash_algorithm: String,
    pub include_source_location: bool,
    pub separator: String,
    pub remove_prefix: Option<String>,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            hash_id: false,
            hash_algorithm: "murmur3".to_string(),
            include_source_location: false,
            separator: ".".to_string(),
            remove_prefix: None,
        }
    }
}

impl ExtractionOptions {
    /// Converts ExtractionOptions to CoreOptions
    fn to_core_options(&self) -> CoreOptions {
        CoreOptions {
            remove_prefix: self
                .remove_prefix
                .as_ref()
                .map(|s| crate::types::RemovePrefix::String(s.clone())),
            separator: self.separator.clone(),
            hash_id: self.hash_id,
            hash_algorithm: self.hash_algorithm.clone(),
            ..Default::default()
        }
    }
}

/// Converts (MessageData, TransformedMessageData) to ExtractedMessage
fn to_extracted_message(
    _message_data: MessageData,
    transformed: TransformedMessageData,
    filename: &PathBuf,
    options: &ExtractionOptions,
    line: Option<usize>,
) -> ExtractedMessage {
    ExtractedMessage {
        id: transformed.id,
        default_message: transformed.default_message,
        description: transformed.description,
        file: if options.include_source_location {
            Some(filename.to_string_lossy().to_string())
        } else {
            None
        },
        line: if options.include_source_location {
            line
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
/// * `options` - Extraction options
///
/// # Returns
/// Vector of extracted messages
///
/// # Example
/// ```
/// use react_intl_core::{extract_messages, ExtractionOptions};
///
/// let code = r#"
/// import { defineMessages } from 'react-intl';
/// export default defineMessages({
///   hello: 'Hello World'
/// });
/// "#;
///
/// let options = ExtractionOptions::default();
/// let messages = extract_messages(code, "test.js", &options);
/// ```
pub fn extract_messages(
    _code: &str,
    _filename: &str,
    _options: &ExtractionOptions,
) -> Vec<ExtractedMessage> {
    // TODO: Implement full parsing with swc_core
    // This requires proper setup of the parser with the correct swc_core version
    // For now, return empty vector - will be implemented with proper parser setup
    vec![]
}

/// Visitor for extracting messages from AST
///
/// This visitor can be used with already parsed AST
pub struct MessageExtractorVisitor {
    messages: Vec<ExtractedMessage>,
    filename: PathBuf,
    options: ExtractionOptions,
    imported_names: std::collections::HashSet<String>,
    alias_map: std::collections::HashMap<String, String>,
    state: CoreState,
}

impl MessageExtractorVisitor {
    pub fn new(filename: PathBuf, options: ExtractionOptions) -> Self {
        let state = CoreState::new(filename.clone(), options.to_core_options());

        Self {
            messages: Vec::new(),
            filename,
            options,
            imported_names: std::collections::HashSet::new(),
            alias_map: std::collections::HashMap::new(),
            state,
        }
    }

    pub fn into_messages(self) -> Vec<ExtractedMessage> {
        self.messages
    }

    fn add_message(
        &mut self,
        message_data: MessageData,
        transformed: TransformedMessageData,
        line: Option<usize>,
    ) {
        let extracted = to_extracted_message(
            message_data,
            transformed,
            &self.filename,
            &self.options,
            line,
        );
        self.messages.push(extracted);
    }
}

impl Visit for MessageExtractorVisitor {
    fn visit_import_decl(&mut self, import: &ImportDecl) {
        // Track React Intl imports
        let source = import.src.value.to_string_lossy();
        if source == "react-intl" || source.starts_with("react-intl/") {
            for specifier in &import.specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local = named.local.sym.to_string();
                        let imported = named
                            .imported
                            .as_ref()
                            .map(|i| match i {
                                ModuleExportName::Ident(ident) => ident.sym.to_string(),
                                _ => local.clone(),
                            })
                            .unwrap_or_else(|| local.clone());

                        self.imported_names.insert(local.clone());
                        if local != imported {
                            self.alias_map.insert(local, imported);
                        }
                    }
                    ImportSpecifier::Default(def) => {
                        self.imported_names.insert(def.local.sym.to_string());
                    }
                    ImportSpecifier::Namespace(ns) => {
                        self.imported_names.insert(ns.local.sym.to_string());
                    }
                }
            }
        }
    }

    fn visit_jsx_element(&mut self, element: &JSXElement) {
        element.visit_children_with(self);

        if let JSXElementName::Ident(name) = &element.opening.name {
            let name_str = name.sym.as_str().to_string();
            let component_name = self.alias_map.get(&name_str).unwrap_or(&name_str);

            if ["FormattedMessage", "FormattedHTMLMessage"].contains(&component_name.as_str()) {
                if let Some((message_data, transformed)) = analyze_jsx_element(element, &self.state)
                {
                    let line = element.span.lo.0; // Approximate line number
                    self.add_message(message_data, transformed, Some(line as usize));
                }
            }
        }
    }

    fn visit_call_expr(&mut self, call: &CallExpr) {
        call.visit_children_with(self);

        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                let fn_name = ident.sym.as_str();

                if fn_name == "defineMessages" && self.imported_names.contains("defineMessages") {
                    let results = analyze_define_messages(call, &self.state);
                    for (message_data, transformed) in results {
                        let line = call.span.lo.0;
                        self.add_message(message_data, transformed, Some(line as usize));
                    }
                } else if fn_name == "formatMessage" {
                    if let Some((message_data, transformed)) =
                        analyze_format_message(call, &self.state)
                    {
                        let line = call.span.lo.0;
                        self.add_message(message_data, transformed, Some(line as usize));
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
    fn test_extraction_options_default() {
        let opts = ExtractionOptions::default();
        assert!(!opts.hash_id);
        assert_eq!(opts.hash_algorithm, "murmur3");
        assert!(!opts.include_source_location);
        assert_eq!(opts.separator, ".");
    }

    #[test]
    fn test_to_extracted_message() {
        let message_data = MessageData {
            id: None,
            default_message: Some("Hello World".to_string()),
            description: Some("A greeting".to_string()),
        };
        let transformed = TransformedMessageData {
            id: "test.hello".to_string(),
            default_message: "Hello World".to_string(),
            description: Some("A greeting".to_string()),
        };
        let filename = PathBuf::from("test.js");
        let opts = ExtractionOptions::default();

        let extracted = to_extracted_message(message_data, transformed, &filename, &opts, Some(42));

        assert_eq!(extracted.id, "test.hello");
        assert_eq!(extracted.default_message, "Hello World");
        assert_eq!(extracted.description, Some("A greeting".to_string()));
        assert!(extracted.line.is_none()); // Not included by default
        assert!(extracted.file.is_none()); // Not included by default
    }

    #[test]
    fn test_to_extracted_message_with_source_location() {
        let message_data = MessageData {
            id: None,
            default_message: Some("Hello".to_string()),
            description: None,
        };
        let transformed = TransformedMessageData {
            id: "test.hello".to_string(),
            default_message: "Hello".to_string(),
            description: None,
        };
        let filename = PathBuf::from("test.js");
        let opts = ExtractionOptions {
            include_source_location: true,
            ..Default::default()
        };

        let extracted = to_extracted_message(message_data, transformed, &filename, &opts, Some(42));

        assert_eq!(extracted.line, Some(42));
        assert!(extracted.file.is_some());
    }
}
