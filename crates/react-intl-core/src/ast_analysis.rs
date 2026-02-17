//! AST analysis functions for React Intl message extraction
//!
//! This module provides functions to analyze AST nodes and extract message data.
//! These functions are shared between the SWC plugin (for transformation) and CLI (for extraction).

use swc_core::ecma::ast::*;

use crate::id_generator::{hash_string, murmur32_hash};
use crate::path_utils::get_prefix;
use crate::types::CoreState;

/// Data structure representing an extracted message from source code
#[derive(Debug, Clone)]
pub struct MessageData {
    pub id: Option<String>,
    pub default_message: Option<String>,
    pub description: Option<String>,
}

/// Data structure representing a transformed message with generated ID
#[derive(Debug, Clone)]
pub struct TransformedMessageData {
    pub id: String,
    pub default_message: String,
    pub description: Option<String>,
}

/// Generates an ID for a message based on the configuration
///
/// # Arguments
/// * `default_message` - The default message text (used for hash if key is not provided)
/// * `key` - Optional key to use as suffix (e.g., message key in defineMessages)
/// * `state` - The core state
/// * `force_use_key` - If true, always use key as suffix regardless of use_key option
pub fn generate_message_id(
    default_message: &str,
    key: Option<&str>,
    state: &CoreState,
    force_use_key: bool,
) -> String {
    let suffix = if force_use_key || state.opts.use_key {
        key.map(|k| k.to_string())
            .unwrap_or_else(|| murmur32_hash(default_message))
    } else {
        murmur32_hash(default_message)
    };

    let prefix = get_prefix(state, Some(&suffix));

    // Apply hash_id option if enabled
    if state.opts.hash_id {
        hash_string(&prefix, &state.opts.hash_algorithm)
    } else {
        prefix
    }
}

/// Analyzes a JSX element and extracts message data if it's a React Intl component
///
/// # Arguments
/// * `element` - The JSX element to analyze
/// * `state` - The core state containing filename and options
/// * `sequence_index` - The sequence index for this JSX element (0, 1, 2, ...)
///
/// # Returns
/// Some((MessageData, TransformedMessageData, bool)) if the element contains a translatable message.
/// The bool indicates whether the ID needs to be inserted (false = ID already exists, true = needs insertion).
/// Returns None if the element doesn't have defaultMessage attribute.
pub fn analyze_jsx_element(
    element: &JSXElement,
    state: &CoreState,
    sequence_index: usize,
) -> Option<(MessageData, TransformedMessageData, bool)> {
    let (id_attr, default_message_attr, description_attr) = extract_jsx_attributes(element);

    // Check if defaultMessage attribute exists at all (even if value is not a static string)
    let has_default_message_attr = element.opening.attrs.iter().any(|attr| {
        if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
            if let JSXAttrName::Ident(name) = &jsx_attr.name {
                return name.sym.as_ref() == "defaultMessage";
            }
        }
        false
    });

    // If there's no defaultMessage attribute at all, this is not a translatable message
    if !has_default_message_attr {
        return None;
    }

    // If there's already an ID, return it as-is without transformation
    if let Some(existing_id) = id_attr {
        let message_data = MessageData {
            id: Some(existing_id.clone()),
            default_message: default_message_attr.clone(),
            description: description_attr.clone(),
        };

        let transformed = TransformedMessageData {
            id: existing_id,
            default_message: default_message_attr.unwrap_or_default(),
            description: description_attr,
        };

        // false = ID already exists, no need to insert
        return Some((message_data, transformed, false));
    }

    // Extract key attribute for use_key option and fallback ID generation
    let key = extract_jsx_key(element);

    // If defaultMessage is a static string, generate ID based on it
    let (id, default_message, message_default_message) = if let Some(msg) = default_message_attr {
        let generated_id = generate_message_id(&msg, key.as_deref(), state, false);
        (generated_id, msg.clone(), Some(msg))
    } else {
        // defaultMessage is a variable/expression - generate fallback ID based on sequence index
        let fallback_id = generate_fallback_jsx_id(key.as_deref(), state, sequence_index);
        (fallback_id, String::new(), None)
    };

    let message_data = MessageData {
        id: None,
        default_message: message_default_message,
        description: description_attr.clone(),
    };

    let transformed = TransformedMessageData {
        id,
        default_message,
        description: description_attr,
    };

    // true = ID needs to be inserted
    Some((message_data, transformed, true))
}

/// Generates a fallback ID for JSX element when defaultMessage is a variable/expression
/// Uses sequence index for consistency
fn generate_fallback_jsx_id(key: Option<&str>, state: &CoreState, sequence_index: usize) -> String {
    // Use key if available and use_key option is enabled
    let suffix = if state.opts.use_key {
        key.map(|k| k.to_string())
    } else {
        None
    };

    // Generate suffix based on sequence index
    let suffix = suffix.unwrap_or_else(|| {
        let file_path = state.filename.to_string_lossy().to_string();
        murmur32_hash(&format!("{}{}", file_path, sequence_index))
    });

    let prefix = get_prefix(state, Some(&suffix));

    // Apply hash_id option if enabled
    if state.opts.hash_id {
        hash_string(&prefix, &state.opts.hash_algorithm)
    } else {
        prefix
    }
}

/// Extracts key attribute from a JSX element
fn extract_jsx_key(element: &JSXElement) -> Option<String> {
    for attr in &element.opening.attrs {
        if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
            if let JSXAttrName::Ident(name) = &jsx_attr.name {
                if name.sym.as_ref() == "key" {
                    return extract_jsx_attr_value(jsx_attr);
                }
            }
        }
    }
    None
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

/// Tries to extract a string value from an expression
fn extract_expr_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(Lit::Str(str_lit)) => Some(str_lit.value.to_string_lossy().to_string()),
        Expr::Tpl(tpl) if tpl.exprs.is_empty() && tpl.quasis.len() == 1 => {
            // Template literal with no expressions: `text`
            let raw = &tpl.quasis[0].raw;
            if raw.is_empty() {
                None
            } else {
                Some(raw.to_string())
            }
        }
        _ => None,
    }
}

/// Analyzes a defineMessages call and extracts all messages
///
/// # Arguments
/// * `call` - The CallExpr for defineMessages
/// * `state` - The core state containing filename and options
/// * `call_index` - The index of this defineMessages call in the file (0, 1, 2, ...)
///
/// # Returns
/// Vector of (key_name, MessageData, TransformedMessageData) for each message that needs transformation
pub fn analyze_define_messages(
    call: &CallExpr,
    state: &CoreState,
    call_index: usize,
) -> Vec<(String, MessageData, TransformedMessageData)> {
    let mut messages = Vec::new();

    // Get the first argument (the object literal)
    if let Some(first_arg) = call.args.first() {
        if let Expr::Object(obj_lit) = first_arg.expr.as_ref() {
            // Use call_index instead of span position for deterministic IDs
            for prop in &obj_lit.props {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Some((key_name, message_data, transformed)) =
                        analyze_object_property_with_key(prop, state, call_index)
                    {
                        messages.push((key_name, message_data, transformed));
                    }
                }
            }
        }
    }

    messages
}

/// Analyzes an object property and extracts message data with key
fn analyze_object_property_with_key(
    prop: &Prop,
    state: &CoreState,
    call_index: usize,
) -> Option<(String, MessageData, TransformedMessageData)> {
    match prop {
        Prop::KeyValue(KeyValueProp { key, value }) => {
            let key_name = extract_prop_name(key)?;

            // Build the full key with call_index prefix for deterministic IDs
            let full_key = format!("{}.{}", call_index, key_name);

            match value.as_ref() {
                // String value: hello: 'Hello World'
                Expr::Lit(Lit::Str(str_lit)) => {
                    let default_message = str_lit.value.to_string_lossy().to_string();

                    let message_data = MessageData {
                        id: None,
                        default_message: Some(default_message.clone()),
                        description: None,
                    };

                    let transformed = TransformedMessageData {
                        id: generate_message_id(&default_message, Some(&full_key), state, true),
                        default_message,
                        description: None,
                    };

                    Some((key_name, message_data, transformed))
                }
                // Object value: hello: { defaultMessage: 'Hello', description: '...' }
                Expr::Object(obj_lit) => {
                    analyze_message_object(obj_lit, Some(&full_key), state, true)
                        .map(|(md, td)| (key_name, md, td))
                }
                // Template literal value: hello: `Hello ${name}`
                Expr::Tpl(_) => {
                    // For template literals, we can't extract the value statically
                    // Generate ID using key_name but with empty default_message
                    // The ID will include the full path prefix via generate_message_id
                    let message_data = MessageData {
                        id: None,
                        default_message: None,
                        description: None,
                    };

                    let transformed = TransformedMessageData {
                        id: generate_message_id("", Some(&full_key), state, true),
                        default_message: String::new(),
                        description: None,
                    };

                    Some((key_name, message_data, transformed))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Extracts property name from PropName
fn extract_prop_name(key: &PropName) -> Option<String> {
    match key {
        PropName::Ident(ident) => Some(ident.sym.to_string()),
        PropName::Str(str_lit) => Some(str_lit.value.to_string_lossy().to_string()),
        _ => None,
    }
}

/// Analyzes a message object (used in defineMessages and formatMessage)
fn analyze_message_object(
    obj_lit: &ObjectLit,
    key_name: Option<&str>,
    state: &CoreState,
    force_use_key: bool,
) -> Option<(MessageData, TransformedMessageData)> {
    analyze_message_object_with_sequence(obj_lit, key_name, state, 0, force_use_key)
}

fn analyze_message_object_with_sequence(
    obj_lit: &ObjectLit,
    key_name: Option<&str>,
    state: &CoreState,
    _sequence_index: usize,
    force_use_key: bool,
) -> Option<(MessageData, TransformedMessageData)> {
    let mut id = None;
    let mut default_message = None;
    let mut description = None;
    let mut has_default_message_attr = false;

    for prop in &obj_lit.props {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() {
                let key_str = extract_prop_name(key)?;

                match key_str.as_str() {
                    "id" => {
                        if let Expr::Lit(Lit::Str(str_lit)) = value.as_ref() {
                            id = Some(str_lit.value.to_string_lossy().to_string());
                        }
                    }
                    "defaultMessage" => {
                        has_default_message_attr = true;
                        if let Some(msg) = extract_expr_string(value) {
                            default_message = Some(msg);
                        }
                    }
                    "description" => {
                        if let Some(desc) = extract_expr_string(value) {
                            description = Some(desc);
                        }
                    }
                    "key" => {
                        // key can be used for ID generation
                        if let Some(_k) = extract_expr_string(value) {
                            // If use_key is enabled, we might use this later
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // If there's already an ID, don't transform (preserve user-defined IDs)
    if id.is_some() {
        return None;
    }

    // If there's no defaultMessage attribute at all, skip
    if !has_default_message_attr {
        return None;
    }

    // Generate ID based on defaultMessage
    // If defaultMessage is not a static string, use the debug representation of the expression
    // This matches the original behavior where format!("{:?}", value) was used
    let (generated_id, transformed_default_message) = if let Some(ref msg) = default_message {
        (
            generate_message_id(msg, key_name, state, force_use_key),
            msg.clone(),
        )
    } else {
        // For non-static defaultMessage (variables, expressions), extract the expression
        // and use its debug representation for ID generation (matching original behavior)
        let debug_str = obj_lit
            .props
            .iter()
            .find_map(|prop| {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() {
                        let key_str = extract_prop_name(key)?;
                        if key_str == "defaultMessage" {
                            // Use debug representation like the original code did
                            return Some(format!("{:?}", value.as_ref()));
                        }
                    }
                }
                None
            })
            .unwrap_or_default();

        if debug_str.is_empty() {
            return None;
        }

        (
            generate_message_id(&debug_str, key_name, state, force_use_key),
            String::new(),
        )
    };

    let message_data = MessageData {
        id: None,
        default_message: default_message.clone(),
        description: description.clone(),
    };

    let transformed = TransformedMessageData {
        id: generated_id,
        default_message: transformed_default_message,
        description,
    };

    Some((message_data, transformed))
}

/// Analyzes a formatMessage call and extracts message data
///
/// # Arguments
/// * `call` - The CallExpr for formatMessage
/// * `state` - The core state containing filename and options
/// * `sequence_index` - The sequence index for this formatMessage call (0, 1, 2, ...)
///
/// # Returns
/// Some((MessageData, TransformedMessageData)) if the call contains a translatable message
/// that needs transformation, None otherwise
pub fn analyze_format_message(
    call: &CallExpr,
    state: &CoreState,
    sequence_index: usize,
) -> Option<(MessageData, TransformedMessageData)> {
    // Get the first argument (the message descriptor object)
    if let Some(first_arg) = call.args.first() {
        if let Expr::Object(obj_lit) = first_arg.expr.as_ref() {
            // For formatMessage, try to extract key from the object for use_key option
            let key = extract_key_from_object(obj_lit);
            return analyze_message_object_with_sequence(
                obj_lit,
                key.as_deref(),
                state,
                sequence_index,
                false, // formatMessage respects use_key option
            );
        }
    }

    None
}

/// Extracts key property from an object literal
fn extract_key_from_object(obj_lit: &ObjectLit) -> Option<String> {
    for prop in &obj_lit.props {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() {
                let key_str = extract_prop_name(key)?;
                if key_str == "key" {
                    return extract_expr_string(value);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let result = analyze_jsx_element(&element, &state, 0);

        assert!(result.is_some());
        let (message_data, transformed, needs_insertion) = result.unwrap();
        assert_eq!(
            message_data.default_message,
            Some("Hello World".to_string())
        );
        assert!(message_data.id.is_none());
        assert!(!transformed.id.is_empty());
        assert_eq!(transformed.default_message, "Hello World");
        assert!(needs_insertion); // ID needs to be inserted
    }

    #[test]
    fn test_analyze_jsx_element_with_existing_id() {
        let code = r#"<FormattedMessage id="my-id" defaultMessage="Hello" />"#;
        let element = parse_jsx(code);
        let state = create_test_state();

        let result = analyze_jsx_element(&element, &state, 0);

        // Should return result with existing ID and needs_insertion=false
        assert!(result.is_some());
        let (message_data, transformed, needs_insertion) = result.unwrap();
        assert_eq!(message_data.id, Some("my-id".to_string()));
        assert_eq!(transformed.id, "my-id");
        assert!(!needs_insertion); // ID already exists, no need to insert
    }

    #[test]
    fn test_analyze_jsx_element_with_variable() {
        let code = r#"<FormattedMessage defaultMessage={message} />"#;
        let element = parse_jsx(code);
        let state = create_test_state();

        let result = analyze_jsx_element(&element, &state, 0);

        // Should return result with fallback ID generated from sequence index
        assert!(result.is_some());
        let (message_data, transformed, needs_insertion) = result.unwrap();
        assert!(message_data.default_message.is_none()); // Variable has no static default_message
        assert!(!transformed.id.is_empty()); // Fallback ID generated
        assert!(needs_insertion); // ID needs to be inserted
    }

    #[test]
    fn test_analyze_define_messages_with_strings() {
        let code = r#"defineMessages({
            hello: 'Hello World',
            goodbye: 'Goodbye'
        })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_define_messages(&call, &state, 0);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "hello");
        assert_eq!(result[1].0, "goodbye");
        assert_eq!(result[0].1.default_message, Some("Hello World".to_string()));
        assert_eq!(result[1].1.default_message, Some("Goodbye".to_string()));
        println!("ID 0: {}", result[0].2.id);
        println!("ID 1: {}", result[1].2.id);
        assert!(!result[0].2.id.is_empty());
        assert!(!result[1].2.id.is_empty());
    }

    #[test]
    fn test_analyze_define_messages_with_objects() {
        let code = r#"defineMessages({
            hello: { defaultMessage: 'Hello World', description: 'A greeting' }
        })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_define_messages(&call, &state, 0);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "hello");
        assert_eq!(result[0].1.default_message, Some("Hello World".to_string()));
        assert_eq!(result[0].1.description, Some("A greeting".to_string()));
        assert_eq!(result[0].2.default_message, "Hello World");
        assert_eq!(result[0].2.description, Some("A greeting".to_string()));
    }

    #[test]
    fn test_analyze_define_messages_with_template_literal() {
        let code = r#"defineMessages({
            hello: `hello world ${1}`
        })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_define_messages(&call, &state, 0);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "hello");
        assert!(result[0].1.default_message.is_none()); // Template literal has no static default_message
        assert!(result[0].2.default_message.is_empty());
        // ID should include the path prefix and key
        println!("Generated ID: {}", result[0].2.id);
        assert!(result[0].2.id.contains("hello"));
        assert!(result[0].2.id.contains("test")); // Should contain path
    }

    #[test]
    fn test_analyze_format_message() {
        let code = r#"formatMessage({ defaultMessage: 'Hello World' })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_format_message(&call, &state, 0);

        assert!(result.is_some());
        let (message_data, transformed) = result.unwrap();
        assert_eq!(
            message_data.default_message,
            Some("Hello World".to_string())
        );
        assert!(!transformed.id.is_empty());
        assert_eq!(transformed.default_message, "Hello World");
    }

    fn parse_call_expr(code: &str) -> CallExpr {
        use swc_core::common::BytePos;
        use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

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
            Expr::Call(call) => call,
            _ => panic!("Expected call expression"),
        }
    }
}
