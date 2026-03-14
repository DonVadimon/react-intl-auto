use react_intl_core::ast::call::{
    analyze_define_messages, analyze_format_message, is_define_messages_call,
    is_format_message_call,
};
use react_intl_core::ast::utils::extract_prop_name;
use react_intl_core::ast::vars::{VarCollector, VarVisitor};
use react_intl_core::types::{CoreState, TransformedMessageData};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use crate::visitors::import::ImportVisitor;

/// Visitor for transforming call expressions (formatMessage, defineMessages)
///
/// Uses VarVisitor to track variable declarations, similar to how
/// ImportVisitor tracks imports. This allows resolution of variable
/// references in calls like `defineMessages(someVar)`.
pub struct CallExpressionVisitor<'a> {
    state: &'a CoreState,
    import_visitor: &'a ImportVisitor<'a>,
    var_visitor: VarVisitor<'a>,
}

impl<'a> CallExpressionVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            import_visitor,
            var_visitor: VarVisitor::new(state),
        }
    }
}

impl<'a> VisitMut for CallExpressionVisitor<'a> {
    fn visit_mut_var_declarator(&mut self, declarator: &mut VarDeclarator) {
        // Track variable declarations using VarVisitor
        // This allows us to resolve variables in function calls
        self.var_visitor.track_declarator(declarator);
        declarator.visit_mut_children_with(self);
    }

    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        // Process formatMessage calls - add ID if missing
        if is_format_message_call(&self.import_visitor, call_expr) && !call_expr.args.is_empty() {
            self.process_format_message(call_expr);
        }

        // Process defineMessages calls - transform the object literal
        if is_define_messages_call(&self.import_visitor, call_expr) && !call_expr.args.is_empty() {
            self.process_define_messages(call_expr);
        }
    }
}

impl<'a> CallExpressionVisitor<'a> {
    fn process_format_message(&mut self, call_expr: &mut CallExpr) {
        // First, analyze the call expression to determine if transformation is needed
        let analysis_result = if let Some(first_arg) = call_expr.args.first() {
            match first_arg.expr.as_ref() {
                // Direct object literal: formatMessage({ defaultMessage: '...' })
                Expr::Object(_) => analyze_format_message(call_expr, self.state),
                // Variable reference: formatMessage(someVar)
                // Resolve the variable using our tracked declarations
                Expr::Ident(ident) => {
                    let var_name = ident.sym.to_string();
                    if let Some(obj_lit) = self.var_visitor.get_object(&var_name) {
                        let call_expr_for_analysis = CallExpr {
                            span: call_expr.span,
                            callee: call_expr.callee.clone(),
                            args: vec![ExprOrSpread {
                                expr: Box::new(Expr::Object(obj_lit.clone())),
                                spread: None,
                            }],
                            type_args: call_expr.type_args.clone(),
                            ctxt: call_expr.ctxt,
                        };
                        analyze_format_message(&call_expr_for_analysis, self.state)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        };

        // Apply transformation if analysis indicates ID needs to be inserted
        if let Some((transformed, need_id_insert)) = analysis_result {
            if need_id_insert {
                if let Some(first_arg) = call_expr.args.first_mut() {
                    if let Expr::Object(obj) = first_arg.expr.as_mut() {
                        // Add the generated ID to the object literal
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
        }
    }

    fn process_define_messages(&mut self, call_expr: &mut CallExpr) {
        // Analyze the call expression to get all messages that need transformation
        let analysis_result =
            analyze_define_messages(call_expr, self.state, Some(&self.var_visitor));

        // Apply transformation if there are messages to process
        if !analysis_result.is_empty() {
            if let Some(first_arg) = call_expr.args.first_mut() {
                match first_arg.expr.as_mut() {
                    // Direct object literal: defineMessages({ hello: '...' })
                    Expr::Object(obj) => {
                        self.apply_define_messages_transformation(obj, analysis_result);
                    }
                    // Variable reference: defineMessages(messages)
                    // Transform the variable's object literal
                    Expr::Ident(ident) => {
                        let var_name = ident.sym.to_string();
                        if let Some(obj_lit) = self.var_visitor.get_object(&var_name) {
                            let mut obj = obj_lit.clone();
                            self.apply_define_messages_transformation(&mut obj, analysis_result);
                            first_arg.expr = Box::new(Expr::Object(obj));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Transforms the object literal in a defineMessages call by adding IDs to each message.
    ///
    /// This handles three cases for message values:
    /// 1. Object values: { defaultMessage: '...', description: '...' } - adds id property
    /// 2. String literals: 'Hello World' - converts to object with id and defaultMessage
    /// 3. Template literals: `Hello ${name}` - converts to object with id and preserved template
    fn apply_define_messages_transformation(
        &self,
        obj: &mut ObjectLit,
        messages: Vec<(String, TransformedMessageData, bool)>,
    ) {
        // Build a map from key name to the transformed data for quick lookup
        let message_id_map: std::collections::HashMap<String, (TransformedMessageData, bool)> =
            messages
                .into_iter()
                .map(|(key_name, transformed, need_id_insert)| {
                    (key_name, (transformed, need_id_insert))
                })
                .collect();

        // Process each property in the object literal
        for prop in &mut obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_mut() {
                    // Extract the key name (e.g., "hello" in { hello: 'world' })
                    let key_name = match extract_prop_name(key) {
                        Some(name) => name,
                        None => continue,
                    };

                    // Look up the analysis result for this key
                    let (transformed, need_id_insert) = match message_id_map.get(&key_name) {
                        Some(data) => data,
                        None => continue,
                    };

                    match value.as_ref() {
                        // Case 1: Value is already an object - just add the id property
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
                        // Case 2: Value is a string literal - convert to object
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
                            // Replace the string with an object containing id and defaultMessage
                            *value = Box::new(Expr::Object(ObjectLit {
                                span: swc_core::common::DUMMY_SP,
                                props: vec![
                                    PropOrSpread::Prop(Box::new(id_prop)),
                                    PropOrSpread::Prop(Box::new(default_message_prop)),
                                ],
                            }));
                        }
                        // Case 3: Value is a template literal - convert to object
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
                            // Replace the template with an object containing id and defaultMessage
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
