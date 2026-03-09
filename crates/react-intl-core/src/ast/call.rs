//! AST analysis functions for React Intl message extraction from defineMessages and formatMessage calls

use swc_core::ecma::ast::*;

use crate::ast::import::ImportCollector;
use crate::ast::utils::{extract_expr_string, extract_prop_name};
use crate::gen::id::{
    generate_message_id, GenIdFromDescriptorPayload, GenIdFromKeyPayload, GenIdPayload,
};
use crate::types::{CoreState, TransformedMessageData};

/// Check call expression is defineMessages call
pub fn is_define_messages_call<T: ImportCollector>(collector: &T, call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Ident(ident) = expr.as_ref() {
            let name = ident.sym.to_string();
            // Check if this function was imported and is defineMessages (or its alias)
            if collector.get_imported_names().contains(&name) {
                // Check if it's defineMessages directly or via alias
                let original_name = collector
                    .get_alias_map()
                    .get(&name)
                    .map(|s| s.as_str())
                    .unwrap_or(name.as_str());
                return original_name == "defineMessages";
            }
        }
    }
    false
}

/// Check call expression is formatMessage call
pub fn is_format_message_call<T: ImportCollector>(collector: &T, call_expr: &CallExpr) -> bool {
    // Check if this is intl.formatMessage call
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Member(member_expr) = expr.as_ref() {
            if let MemberProp::Ident(prop) = &member_expr.prop {
                // Check if this is formatMessage call
                if prop.sym == "formatMessage" {
                    return true;
                }
            }
        }
    }

    // Check if this is a direct call to formatMessage (not defineMessages)
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Ident(ident) = expr.as_ref() {
            let name = ident.sym.to_string();
            // Check if this function was imported and is formatMessage (or its alias)
            if collector.get_imported_names().contains(&name) {
                // Check if it's formatMessage directly or via alias
                let original_name = collector
                    .get_alias_map()
                    .get(&name)
                    .map(|s| s.as_str())
                    .unwrap_or(name.as_str());
                return original_name == "formatMessage";
            }
        }
    }

    false
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CoreOptions;

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
