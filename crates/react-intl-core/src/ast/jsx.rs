//! AST analysis functions for React Intl message extraction from jsx elements

use swc_core::ecma::ast::*;

use crate::ast::import::ImportVisitor;
use crate::ast::utils::extract_expr_string;
use crate::gen::id::{generate_message_id, GenIdFromDescriptorPayload, GenIdPayload};
use crate::types::{CoreState, TransformedMessageData, REACT_COMPONENTS};

/// Check if a JSX element is a React Intl component
///
/// This function checks if the element name is in the list of React Intl components
/// and if it was imported from the react-intl module.
///
/// # Arguments
/// * `import_visitor` - The import visitor containing imported names and aliases
/// * `name` - The identifier of the JSX element
///
/// # Returns
/// `true` if the element is a React Intl component, `false` otherwise
pub fn is_react_intl_component(import_visitor: &ImportVisitor, name: &Ident) -> bool {
    let name_str = name.sym.as_str().to_string();
    let component_name = import_visitor.alias_map.get(&name_str).unwrap_or(&name_str);

    REACT_COMPONENTS.contains(&component_name.as_str())
        && import_visitor.imported_names.contains(&name_str)
}

/// Analyzes a JSX element and extracts message data if it's a React Intl component
///
/// # Arguments
/// * `element` - The JSX element to analyze
/// * `state` - The core state containing filename and options
///
/// # Returns
/// Some((TransformedMessageData, bool)) if the element contains a translatable message.
/// The bool indicates whether the ID needs to be inserted (false = ID already exists, true = needs insertion).
/// Returns None if the element can't be translated.
pub fn analyze_jsx_element(
    element: &JSXElement,
    state: &CoreState,
) -> Option<(TransformedMessageData, bool)> {
    let (id_attr, default_message_attr, description_attr) = extract_jsx_attributes(element);

    // If there's already an ID, return it as-is without transformation
    if let Some(existing_id) = id_attr {
        let transformed = TransformedMessageData {
            id: existing_id,
            default_message: default_message_attr,
            description: description_attr,
        };

        // false = ID already exists, no need to insert
        return Some((transformed, false));
    }

    // If there's no defaultMessage attribute at all or it is not statically
    // evaluated as a string, this is not a translatable message
    let default_message = if let Some(default_message) = &default_message_attr {
        default_message
    } else {
        return None;
    };

    // generate ID based on attrs
    let payload = GenIdPayload::Descriptor(GenIdFromDescriptorPayload {
        default_message: &default_message,
        description: &description_attr,
    });
    let generated_id = generate_message_id(state, &payload);

    let transformed = TransformedMessageData {
        id: generated_id,
        default_message: default_message_attr,
        description: description_attr,
    };

    // true = ID needs to be inserted
    Some((transformed, true))
}

/// Extracts attributes from a JSX element
///
/// Returns tuple of (id, defaultMessage, description)
fn extract_jsx_attributes(
    element: &JSXElement,
) -> (Option<String>, Option<String>, Option<String>) {
    let mut id = None;
    let mut default_message = None;
    let mut description = None;

    for attr in &element.opening.attrs {
        if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
            if let JSXAttrName::Ident(name) = &jsx_attr.name {
                match name.sym.as_ref() {
                    "id" => {
                        id = extract_jsx_attr_value(jsx_attr);
                    }
                    "defaultMessage" => {
                        default_message = extract_jsx_attr_value(jsx_attr);
                    }
                    "description" => {
                        description = extract_jsx_attr_value(jsx_attr);
                    }
                    _ => {}
                }
            }
        }
    }

    (id, default_message, description)
}

/// Extracts string value from a JSX attribute
fn extract_jsx_attr_value(jsx_attr: &JSXAttr) -> Option<String> {
    match &jsx_attr.value {
        Some(JSXAttrValue::Str(str_lit)) => Some(str_lit.value.to_string_lossy().to_string()),
        Some(JSXAttrValue::JSXExprContainer(JSXExprContainer { expr, .. })) => match expr {
            JSXExpr::Expr(expr) => extract_expr_string(expr),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen::id::hash_string;
    use crate::types::CoreOptions;
    use swc_core::common::BytePos;
    use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

    fn create_test_state() -> CoreState {
        CoreState::new(
            std::path::PathBuf::from("/test/file.js"),
            CoreOptions {
                remove_prefix: Some(crate::types::RemovePrefix::String("test".to_string())),
                separator: ".".to_string(),
                ..Default::default()
            },
        )
    }

    fn parse_jsx(code: &str) -> Box<JSXElement> {
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                tsx: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::new(code, BytePos(0), BytePos(code.len() as u32)),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let expr = parser.parse_expr().expect("Failed to parse");

        match *expr {
            Expr::JSXElement(element) => element,
            _ => panic!("Expected JSX element"),
        }
    }

    #[test]
    fn test_analyze_jsx_element_with_default_message() {
        let code = r#"<FormattedMessage defaultMessage="Hello World" />"#;
        let element = parse_jsx(code);
        let state = create_test_state();

        let result = analyze_jsx_element(&element, &state);

        assert!(result.is_some());
        let (transformed, needs_insertion) = result.unwrap();
        assert!(!transformed.id.is_empty());
        assert_eq!(transformed.default_message.unwrap(), "Hello World");
        assert!(needs_insertion); // ID needs to be inserted
    }

    #[test]
    fn test_analyze_jsx_element_with_existing_id() {
        let code = r#"<FormattedMessage id="my-id" defaultMessage="Hello" />"#;
        let element = parse_jsx(code);
        let state = create_test_state();

        let result = analyze_jsx_element(&element, &state);

        // Should return result with existing ID and needs_insertion=false
        assert!(result.is_some());
        let (transformed, needs_insertion) = result.unwrap();
        assert_eq!(transformed.id, "my-id");
        assert!(!needs_insertion); // ID already exists, no need to insert
    }

    #[test]
    fn test_analyze_jsx_element_with_variable() {
        let code = r#"<FormattedMessage defaultMessage={message} />"#;
        let element = parse_jsx(code);
        let state = create_test_state();

        let result = analyze_jsx_element(&element, &state);

        // Should return None for id that is not statically evaluated
        assert!(result.is_none());
    }

    #[test]
    fn test_analyze_jsx_element_with_jsx_expr() {
        let code = r#"<FormattedMessage defaultMessage={'message'} />"#;
        let element = parse_jsx(code);
        let state = create_test_state();

        let result = analyze_jsx_element(&element, &state);

        // Should return some for attrs that wrapped in jsx expr
        assert!(result.is_some());
        let (transformed, needs_insertion) = result.unwrap();
        assert!(transformed
            .id
            .contains(hash_string("message", "murmur3").as_str()));
        assert!(needs_insertion); // ID needs to be inserted
    }

    #[test]
    fn test_analyze_jsx_element_with_string_literals() {
        let code = r#"<FormattedMessage defaultMessage={`message`} />"#;
        let element = parse_jsx(code);
        let state = create_test_state();

        let result = analyze_jsx_element(&element, &state);

        // Should return some for attrs that wrapped in string literals
        assert!(result.is_some());
        let (transformed, needs_insertion) = result.unwrap();
        assert!(transformed
            .id
            .contains(hash_string("message", "murmur3").as_str()));
        assert!(needs_insertion); // ID needs to be inserted
    }
}
