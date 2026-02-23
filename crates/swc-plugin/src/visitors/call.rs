use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use react_intl_core::{
    analyze_define_messages, analyze_format_message, extract_prop_name, TransformedMessageData,
};

use react_intl_core::CoreState;

use crate::visitors::import::ImportVisitor;

pub struct CallExpressionVisitor<'a> {
    pub state: &'a CoreState,
    pub imported_names: &'a std::collections::HashSet<String>,
    pub alias_map: &'a std::collections::HashMap<String, String>,
    pub variable_declarations: std::collections::HashMap<String, ObjectLit>,
}

impl<'a> VisitMut for CallExpressionVisitor<'a> {
    fn visit_mut_var_declarator(&mut self, declarator: &mut VarDeclarator) {
        // Track variable declarations with object literals
        if let Pat::Ident(ident) = &declarator.name {
            if let Some(init) = &declarator.init {
                if let Expr::Object(obj) = init.as_ref() {
                    self.variable_declarations
                        .insert(ident.sym.to_string(), obj.clone());
                }
            }
        }

        declarator.visit_mut_children_with(self);
    }

    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        self.add_id_to_format_message(call_expr);
        self.add_id_to_define_message(call_expr);
    }
}

impl<'a> CallExpressionVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            imported_names: &import_visitor.imported_names,
            alias_map: &import_visitor.alias_map,
            variable_declarations: std::collections::HashMap::new(),
        }
    }

    fn is_format_message_call(&self, call_expr: &CallExpr) -> bool {
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
                if self.imported_names.contains(&name) {
                    // Check if it's formatMessage directly or via alias
                    let original_name = self
                        .alias_map
                        .get(&name)
                        .map(|s| s.as_str())
                        .unwrap_or(name.as_str());
                    return original_name == "formatMessage";
                }
            }
        }

        false
    }

    fn is_define_messages_call(&self, call_expr: &CallExpr) -> bool {
        if let Callee::Expr(expr) = &call_expr.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                let name = ident.sym.to_string();
                // Check if this function was imported and is defineMessages (or its alias)
                if self.imported_names.contains(&name) {
                    // Check if it's defineMessages directly or via alias
                    let original_name = self
                        .alias_map
                        .get(&name)
                        .map(|s| s.as_str())
                        .unwrap_or(name.as_str());
                    return original_name == "defineMessages";
                }
            }
        }
        false
    }

    fn add_id_to_format_message(&mut self, call_expr: &mut CallExpr) {
        if !self.is_format_message_call(call_expr) {
            return;
        }

        if call_expr.args.is_empty() {
            return;
        }

        let first_arg = &mut call_expr.args[0];
        let ExprOrSpread { expr, .. } = first_arg;

        match expr.as_mut() {
            Expr::Object(obj) => {
                // Create a call_expr for analysis
                let call_expr_for_analysis = CallExpr {
                    span: call_expr.span,
                    callee: call_expr.callee.clone(),
                    args: vec![ExprOrSpread {
                        expr: Box::new(Expr::Object(obj.clone())),
                        spread: None,
                    }],
                    type_args: call_expr.type_args.clone(),
                    ctxt: call_expr.ctxt,
                };
                self.process_format_message_object_with_analysis(obj, &call_expr_for_analysis);
            }
            Expr::Ident(ident) => {
                // Handle variable reference
                let var_name = ident.sym.to_string();
                if let Some(obj_lit) = self.variable_declarations.get(&var_name).cloned() {
                    let mut obj = obj_lit;
                    // Create a call_expr for analysis
                    let call_expr_for_analysis = CallExpr {
                        span: call_expr.span,
                        callee: call_expr.callee.clone(),
                        args: vec![ExprOrSpread {
                            expr: Box::new(Expr::Object(obj.clone())),
                            spread: None,
                        }],
                        type_args: call_expr.type_args.clone(),
                        ctxt: call_expr.ctxt,
                    };
                    self.process_format_message_object_with_analysis(
                        &mut obj,
                        &call_expr_for_analysis,
                    );
                    *expr = Box::new(Expr::Object(obj));
                }
            }
            _ => {}
        }
    }

    fn add_id_to_define_message(&mut self, call_expr: &mut CallExpr) {
        if !self.is_define_messages_call(call_expr) {
            return;
        }

        if call_expr.args.is_empty() {
            return;
        }

        let first_arg = &mut call_expr.args[0];
        let ExprOrSpread { expr, .. } = first_arg;
        match expr.as_mut() {
            Expr::Object(obj) => {
                // Check if object has "id" property at the top level (indicates already processed)
                let has_id_at_top_level = obj.props.iter().any(|prop| {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(KeyValueProp { key, .. }) = prop.as_ref() {
                            match key {
                                PropName::Ident(ident) => return ident.sym == "id",
                                PropName::Str(str_lit) => return str_lit.value == "id",
                                _ => {}
                            }
                        }
                    }
                    false
                });

                if has_id_at_top_level {
                    // Already processed, skip
                    return;
                }

                // Pass the actual call_expr to analyze_define_messages
                let call_expr_for_analysis = CallExpr {
                    span: call_expr.span,
                    callee: call_expr.callee.clone(),
                    args: vec![ExprOrSpread {
                        expr: Box::new(Expr::Object(obj.clone())),
                        spread: None,
                    }],
                    type_args: call_expr.type_args.clone(),
                    ctxt: call_expr.ctxt,
                };
                self.process_define_messages_object_with_analysis(obj, &call_expr_for_analysis);
            }
            Expr::Ident(ident) => {
                // Handle variable reference
                let var_name = ident.sym.to_string();
                if let Some(obj_lit) = self.variable_declarations.get(&var_name).cloned() {
                    let mut obj = obj_lit;
                    // Pass the actual call_expr to analyze_define_messages
                    let call_expr_for_analysis = CallExpr {
                        span: call_expr.span,
                        callee: call_expr.callee.clone(),
                        args: vec![ExprOrSpread {
                            expr: Box::new(Expr::Object(obj.clone())),
                            spread: None,
                        }],
                        type_args: call_expr.type_args.clone(),
                        ctxt: call_expr.ctxt,
                    };
                    self.process_define_messages_object_with_analysis(
                        &mut obj,
                        &call_expr_for_analysis,
                    );
                    *expr = Box::new(Expr::Object(obj));
                }
            }
            _ => {}
        }
    }

    fn process_format_message_object_with_analysis(
        &mut self,
        obj: &mut ObjectLit,
        call_expr: &CallExpr,
    ) {
        if let Some((transformed, need_id_insert)) = analyze_format_message(call_expr, &self.state)
        {
            if need_id_insert {
                // Add id property using the ID generated by core crate
                let id_prop = object_property(
                    "id",
                    Expr::Lit(Lit::Str(Str {
                        span: swc_core::common::DUMMY_SP,
                        value: transformed.id.into(),
                        raw: None,
                    })),
                );

                obj.props.push(PropOrSpread::Prop(Box::new(id_prop)));
            }
        }
    }

    fn process_define_messages_object_with_analysis(
        &mut self,
        obj: &mut ObjectLit,
        call_expr: &CallExpr,
    ) {
        // Use the shared core function to analyze defineMessages
        let messages = analyze_define_messages(call_expr, &self.state);

        if messages.is_empty() {
            // no messages - do nothing
            return;
        }

        // Build a map from key name to the transformed ID
        // This uses the ID generated by the shared core crate
        let message_id_map: std::collections::HashMap<String, (TransformedMessageData, bool)> =
            messages
                .into_iter()
                .map(|(key_name, transformed, need_id_insert)| {
                    (key_name, (transformed, need_id_insert))
                })
                .collect();

        // Update object properties based on analysis
        for prop in &mut obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_mut() {
                    // Extract the key name from the property
                    let key_name = extract_prop_name(key);

                    let Some(key_name) = key_name else {
                        continue;
                    };

                    // Get the pre-generated ID from the shared core analysis
                    let Some((transformed, need_id_insert)) = message_id_map.get(&key_name) else {
                        continue;
                    };

                    match value.as_ref() {
                        // Add id prop to existing object
                        Expr::Object(_) => {
                            if *need_id_insert {
                                let id_prop = object_property(
                                    "id",
                                    Expr::Lit(Lit::Str(Str {
                                        span: swc_core::common::DUMMY_SP,
                                        value: transformed.id.clone().into(),
                                        raw: None,
                                    })),
                                );

                                if let Expr::Object(inner_obj) = value.as_mut() {
                                    inner_obj.props.push(PropOrSpread::Prop(Box::new(id_prop)));
                                }
                            }
                        }
                        // Convert string to object with id and defaultMessage
                        Expr::Lit(Lit::Str(str_lit)) => {
                            let id_prop = object_property(
                                "id",
                                Expr::Lit(Lit::Str(Str {
                                    span: swc_core::common::DUMMY_SP,
                                    value: transformed.id.clone().into(),
                                    raw: None,
                                })),
                            );

                            let default_message_prop = object_property(
                                "defaultMessage",
                                Expr::Lit(Lit::Str(Str {
                                    span: swc_core::common::DUMMY_SP,
                                    value: str_lit.value.clone(),
                                    raw: None,
                                })),
                            );

                            *value = Box::new(Expr::Object(ObjectLit {
                                span: swc_core::common::DUMMY_SP,
                                props: vec![
                                    PropOrSpread::Prop(Box::new(id_prop)),
                                    PropOrSpread::Prop(Box::new(default_message_prop)),
                                ],
                            }));
                        }
                        // Convert template literal to object with id and preserved template
                        Expr::Tpl(template) => {
                            let id_prop = object_property(
                                "id",
                                Expr::Lit(Lit::Str(Str {
                                    span: swc_core::common::DUMMY_SP,
                                    value: transformed.id.clone().into(),
                                    raw: None,
                                })),
                            );

                            let default_message_prop =
                                object_property("defaultMessage", Expr::Tpl(template.clone()));

                            *value = Box::new(Expr::Object(ObjectLit {
                                span: swc_core::common::DUMMY_SP,
                                props: vec![
                                    PropOrSpread::Prop(Box::new(id_prop)),
                                    PropOrSpread::Prop(Box::new(default_message_prop)),
                                ],
                            }));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Creates an object property for use in AST transformations
///
/// This is a SWC-specific utility that remains in the plugin crate
/// because it deals with SWC AST types.
fn object_property(key: &str, value: Expr) -> Prop {
    Prop::KeyValue(KeyValueProp {
        key: PropName::Str(Str {
            span: swc_core::common::DUMMY_SP,
            value: key.into(),
            raw: None,
        }),
        value: Box::new(value),
    })
}
