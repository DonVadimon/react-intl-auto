use react_intl_core::ast::call::{
    analyze_define_messages, analyze_format_message, is_define_messages_call,
    is_format_message_call,
};

use react_intl_core::types::{CoreState, TransformedMessageData};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::visitors::import::ImportVisitor;

pub struct CallExpressionVisitor<'a> {
    pub state: &'a CoreState,
    pub messages: Vec<TransformedMessageData>,
    import_visitor: &'a ImportVisitor<'a>,
    variable_declarations: std::collections::HashMap<String, ObjectLit>,
}

impl<'a> Visit for CallExpressionVisitor<'a> {
    fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
        // Track variable declarations with object literals
        if let Pat::Ident(ident) = &declarator.name {
            if let Some(init) = &declarator.init {
                if let Expr::Object(obj) = init.as_ref() {
                    self.variable_declarations
                        .insert(ident.sym.to_string(), obj.clone());
                }
            }
        }

        declarator.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        call_expr.visit_children_with(self);

        self.add_id_to_format_message(call_expr);
        self.add_id_to_define_message(call_expr);
    }
}

impl<'a> CallExpressionVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            import_visitor,
            variable_declarations: std::collections::HashMap::new(),
            messages: Vec::new(),
        }
    }

    fn add_id_to_format_message(&mut self, call_expr: &CallExpr) {
        if !is_format_message_call(&self.import_visitor, call_expr) {
            return;
        }

        if call_expr.args.is_empty() {
            return;
        }

        let first_arg = &call_expr.args[0];
        let ExprOrSpread { expr, .. } = first_arg;

        match expr.as_ref() {
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
                self.process_format_message_object_with_analysis(&call_expr_for_analysis);
            }
            Expr::Ident(ident) => {
                // Handle variable reference
                let var_name = ident.sym.to_string();
                if let Some(obj_lit) = self.variable_declarations.get(&var_name).cloned() {
                    // Create a call_expr for analysis
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
                    self.process_format_message_object_with_analysis(&call_expr_for_analysis);
                }
            }
            _ => {}
        }
    }

    fn add_id_to_define_message(&mut self, call_expr: &CallExpr) {
        if !is_define_messages_call(&self.import_visitor, call_expr) {
            return;
        }

        if call_expr.args.is_empty() {
            return;
        }

        let first_arg = &call_expr.args[0];
        let ExprOrSpread { expr, .. } = first_arg;
        match expr.as_ref() {
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
                self.process_define_messages_object_with_analysis(&call_expr_for_analysis);
            }
            Expr::Ident(ident) => {
                // Handle variable reference
                let var_name = ident.sym.to_string();
                if let Some(obj_lit) = self.variable_declarations.get(&var_name).cloned() {
                    // Pass the actual call_expr to analyze_define_messages
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
                    self.process_define_messages_object_with_analysis(&call_expr_for_analysis);
                }
            }
            _ => {}
        }
    }

    fn process_format_message_object_with_analysis(&mut self, call_expr: &CallExpr) {
        if let Some((transformed, _need_id_insert)) = analyze_format_message(call_expr, &self.state)
        {
            self.messages.push(transformed);
        }
    }

    fn process_define_messages_object_with_analysis(&mut self, call_expr: &CallExpr) {
        // Use the shared core function to analyze defineMessages
        let messages = analyze_define_messages(call_expr, &self.state);

        if messages.is_empty() {
            // no messages - do nothing
            return;
        }

        let mut transformed: Vec<_> = messages
            .into_iter()
            .map(|(_key_name, transformed, _need_id_insert)| transformed)
            .collect();

        self.messages.append(&mut transformed);
    }
}
