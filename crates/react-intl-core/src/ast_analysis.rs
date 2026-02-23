//! AST analysis functions for React Intl message extraction
//!
//! This module provides functions to analyze AST nodes and extract message data.
//! These functions are shared between the SWC plugin (for transformation) and CLI (for extraction).

use swc_core::ecma::ast::*;

use crate::id_generator::{hash_string, murmur32_hash};
use crate::path_utils::add_prefix;
use crate::types::CoreState;

/// Data structure representing a transformed message with generated ID
#[derive(Debug, Clone)]
pub struct TransformedMessageData {
    pub id: String,
    pub default_message: Option<String>,
    pub description: Option<String>,
}

/// payload to generate id from message description
/// object
/// ```js
/// const messages = intl.formatMessage({
///     defaultMessage: "defaultMessage", // hash default message
///     description: "description", // + hash description
/// });
/// ```
/// jsx
/// ```js
/// <FormattedMessage defaultMessage="hello" /> // hash default message
/// ```
#[derive(Debug, Clone)]
struct GenIdFromDescriptorPayload<'a> {
    pub default_message: &'a String,
    pub description: &'a Option<String>,
}

/// payload to generate id from key + part of message description
/// only key
/// ```js
/// const messages = defineMessages({
///     hello: { // key = "hello"
///         defaultMessage: "defaultMessage",
///         description: "description", // + hash description
///     }
/// });
/// ```
/// key + descriptor path
/// ```js
/// const messages = defineMessages({
///     hello: { // key = "hello"
///         defaultMessage: "defaultMessage",
///         description: "description", // + hash description
///     }
/// });
/// ```
#[derive(Debug, Clone)]
struct GenIdFromKeyPayload<'a> {
    pub key: &'a String,
    pub description: &'a Option<String>,
}

#[derive(Debug, Clone)]
enum GenIdPayload<'a> {
    Key(GenIdFromKeyPayload<'a>),
    Descriptor(GenIdFromDescriptorPayload<'a>),
}

/// Generates an ID for a message based on the configuration
///
/// # Arguments
/// * `state` - The core state
/// * `payload` - Payload for id generation
fn generate_message_id(state: &CoreState, payload: &GenIdPayload) -> String {
    let raw_id = match payload {
        GenIdPayload::Key(key_payload) => {
            let mut parts = vec![key_payload.key.to_owned()];
            if let Some(description) = &key_payload.description {
                parts.push(murmur32_hash(description.as_str()));
            }
            parts.join(state.opts.separator.as_str())
        }
        GenIdPayload::Descriptor(descriptor) => {
            let mut parts = vec![descriptor.default_message.to_owned()];
            if let Some(description) = &descriptor.description {
                parts.push(description.to_owned());
            }
            murmur32_hash(parts.join(state.opts.separator.as_str()).as_str())
        }
    };

    let path_id = add_prefix(state, &raw_id);

    // Apply hash_id option if enabled
    if state.opts.hash_id {
        hash_string(&path_id, &state.opts.hash_algorithm)
    } else {
        path_id
    }
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

/// Tries to extract a string value from an expression
///
/// Supports:
/// * string literals
/// ```js
/// 'hello' // "hello"
/// ```
/// * template strings
/// ```js
/// `hello world` // ok
/// `hello ${world}` // no support
/// ```
fn extract_expr_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(Lit::Str(str_lit)) => Some(str_lit.value.to_string_lossy().to_string()),
        Expr::Tpl(tpl) if tpl.exprs.is_empty() && tpl.quasis.len() == 1 => {
            // Template literal with no expressions: `text`
            // TODO: use evaluator maybe https://rustdoc.swc.rs/swc_ecma_minifier/eval/struct.Evaluator.html
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
///
/// # Returns
/// Vector of (key_name, TransformedMessageData, need_id_insert) for each message that needs transformation
pub fn analyze_define_messages(
    call: &CallExpr,
    state: &CoreState,
) -> Vec<(String, TransformedMessageData, bool)> {
    let mut messages = Vec::new();

    // Get the first argument (the object literal)
    if let Some(first_arg) = call.args.first() {
        if let Expr::Object(obj_lit) = first_arg.expr.as_ref() {
            // Use call_index instead of span position for deterministic IDs
            for prop in &obj_lit.props {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Some((key_name, message_data, transformed)) =
                        analyze_define_messages_object_property(prop, state)
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
/// ```js
/// defineMessages({
///     hello: { // <- this object
///         defaultMessage: 'defaultMessage',
///         description: 'description',
///     }
/// })
/// ```
/// or
/// ```js
/// defineMessages({
///     hello: 'world' // <- this string
/// })
/// ```
///
/// # Returns
/// `Some((String, TransformedMessageData, bool))` if the prop contains a translatable message
/// The bool indicates whether the ID needs to be inserted (false = ID already exists, true = needs insertion).
/// Returns None if the property can't be translated.
fn analyze_define_messages_object_property(
    prop: &Prop,
    state: &CoreState,
) -> Option<(String, TransformedMessageData, bool)> {
    match prop {
        Prop::KeyValue(KeyValueProp { key, value }) => {
            let key_name = extract_prop_name(key)?;

            match value.as_ref() {
                // Object value: hello: { defaultMessage: 'Hello', description: '...' }
                Expr::Object(obj_lit) => analyze_message_object(obj_lit, state, Some(&key_name))
                    .map(|(md, td)| (key_name, md, td)),
                // String value: hello: 'Hello World'
                // Template literal value: hello: `Hello ${name}`
                _ => {
                    // For template literals, we can try to extract the value statically
                    // But we have some limitations
                    let default_message_prop = extract_expr_string(value);

                    if default_message_prop.is_some() {
                        let payload = GenIdPayload::Key(GenIdFromKeyPayload {
                            key: &key_name,
                            description: &None,
                        });

                        let transformed = TransformedMessageData {
                            id: generate_message_id(state, &payload),
                            default_message: default_message_prop,
                            description: None,
                        };

                        // true = ID needs to be inserted
                        return Some((key_name, transformed, true));
                    }

                    None
                }
            }
        }
        _ => None,
    }
}

/// Extracts property name from PropName
pub fn extract_prop_name(key: &PropName) -> Option<String> {
    match key {
        PropName::Ident(ident) => Some(ident.sym.to_string()),
        PropName::Str(str_lit) => Some(str_lit.value.to_string_lossy().to_string()),
        PropName::Num(num_lit) => Some(num_lit.value.to_string()),
        _ => None,
    }
}

/// Analyzes a message object (used in defineMessages and formatMessage)
/// ```js
/// defineMessages({
///     hello: { // <- this object
///         defaultMessage: 'defaultMessage',
///         description: 'description',
///     }
/// })
/// ```
///
/// # Arguments
/// * `obj_lit` - The ObjectLit with id?, defaultMessage?, description?
/// * `state` - The core state containing filename and options
/// * `key` - Optional key of parent object where `obj_lit` is located (in example - "hello")
///
/// # Returns
/// `Some((TransformedMessageData, bool))` if the obj_lit contains a translatable message
/// The bool indicates whether the ID needs to be inserted (false = ID already exists, true = needs insertion).
/// Returns None if the obj_lit can't be translated.
fn analyze_message_object(
    obj_lit: &ObjectLit,
    state: &CoreState,
    key: Option<&str>,
) -> Option<(TransformedMessageData, bool)> {
    let mut id_prop = None;
    let mut default_message_prop = None;
    let mut description_prop = None;

    for prop in &obj_lit.props {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() {
                let key_str = extract_prop_name(key)?;

                match key_str.as_str() {
                    "id" => {
                        id_prop = extract_expr_string(value);
                    }
                    "defaultMessage" => {
                        default_message_prop = extract_expr_string(value);
                    }
                    "description" => {
                        description_prop = extract_expr_string(value);
                    }
                    _ => {}
                }
            }
        }
    }

    // If there's already an ID, return it as-is without transformation
    if let Some(existing_id) = id_prop {
        let transformed = TransformedMessageData {
            id: existing_id,
            default_message: default_message_prop,
            description: description_prop,
        };

        // false = ID already exists, no need to insert
        return Some((transformed, false));
    }

    // If there's no defaultMessage attribute at all or it is not statically
    // evaluated as a string, this is not a translatable message
    let default_message = if let Some(default_message) = &default_message_prop {
        default_message
    } else {
        return None;
    };

    // if key provided - use key based id generation
    // else - use object props based id generation
    let payload = match key {
        Some(key) => GenIdPayload::Key(GenIdFromKeyPayload {
            key: &key.to_string(),
            description: &description_prop,
        }),
        None => GenIdPayload::Descriptor(GenIdFromDescriptorPayload {
            default_message: &default_message,
            description: &description_prop,
        }),
    };
    let generated_id = generate_message_id(state, &payload);

    let transformed = TransformedMessageData {
        id: generated_id,
        default_message: default_message_prop,
        description: description_prop,
    };

    // true = ID needs to be inserted
    Some((transformed, true))
}

/// Analyzes a formatMessage call and extracts message data
///
/// # Arguments
/// * `call` - The CallExpr for formatMessage
/// * `state` - The core state containing filename and options
///
/// # Returns
/// `Some((TransformedMessageData, bool))` if the call contains a translatable message
/// The bool indicates whether the ID needs to be inserted (false = ID already exists, true = needs insertion).
/// Returns None if the call can't be translated.
pub fn analyze_format_message(
    call: &CallExpr,
    state: &CoreState,
) -> Option<(TransformedMessageData, bool)> {
    // Get the first argument (the message descriptor object)
    if let Some(first_arg) = call.args.first() {
        if let Expr::Object(obj_lit) = first_arg.expr.as_ref() {
            return analyze_message_object(obj_lit, state, None);
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
        assert!(transformed.id.contains(murmur32_hash("message").as_str()));
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
        assert!(transformed.id.contains(murmur32_hash("message").as_str()));
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

        let result = analyze_define_messages(&call, &state);
        assert_eq!(result.len(), 2);

        let hello_msg = &result[0];
        let goodbye_msg = &result[1];

        assert_eq!(hello_msg.0, "hello");
        assert_eq!(goodbye_msg.0, "goodbye");

        assert_eq!(hello_msg.1.default_message, Some("Hello World".to_string()));
        assert_eq!(goodbye_msg.1.default_message, Some("Goodbye".to_string()));

        assert!(!hello_msg.1.id.is_empty());
        assert!(!goodbye_msg.1.id.is_empty());

        assert!(hello_msg.2);
        assert!(goodbye_msg.2);
    }

    #[test]
    fn test_analyze_define_messages_with_objects() {
        let code = r#"defineMessages({
            hello: { defaultMessage: 'Hello World', description: 'A greeting' }
        })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_define_messages(&call, &state);
        assert_eq!(result.len(), 1);

        let hello_msg = &result[0];

        assert_eq!(hello_msg.0, "hello");

        assert_eq!(hello_msg.1.default_message, Some("Hello World".to_string()));
        assert_eq!(hello_msg.1.description, Some("A greeting".to_string()));

        assert!(hello_msg.2);
    }

    #[test]
    fn test_analyze_define_messages_with_template_literal() {
        let code = r#"defineMessages({
            hello: `hello world`
        })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_define_messages(&call, &state);
        assert_eq!(result.len(), 1);

        let hello_msg = &result[0];

        assert_eq!(hello_msg.0, "hello");
        assert!(hello_msg.1.id.contains("hello"));
        assert!(hello_msg.1.default_message.is_some());
        assert!(!hello_msg.1.default_message.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_analyze_define_messages_with_non_evaluatable_template_literal() {
        let code = r#"defineMessages({
            hello: `hello ${world}`
        })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_define_messages(&call, &state);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_analyze_format_message() {
        let code = r#"formatMessage({ defaultMessage: 'Hello World' })"#;
        let call = parse_call_expr(code);
        let state = create_test_state();

        let result = analyze_format_message(&call, &state);

        assert!(result.is_some());
        let (transformed, need_id_insert) = result.unwrap();
        assert!(!transformed.id.is_empty());
        assert_eq!(transformed.default_message, Some("Hello World".to_string()));
        assert!(need_id_insert);
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
