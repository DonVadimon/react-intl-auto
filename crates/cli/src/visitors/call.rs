use react_intl_core::ast::call::{analyze_define_messages, analyze_format_message};
use react_intl_core::types::{CoreState, TransformedMessageData};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::visitors::import::ImportVisitor;

pub struct CallExpressionVisitor<'a> {
    pub state: &'a CoreState,
    pub imported_names: &'a std::collections::HashSet<String>,
    pub alias_map: &'a std::collections::HashMap<String, String>,
    pub variable_declarations: std::collections::HashMap<String, ObjectLit>,
    pub messages: Vec<TransformedMessageData>,
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
            imported_names: &import_visitor.imported_names,
            alias_map: &import_visitor.alias_map,
            variable_declarations: std::collections::HashMap::new(),
            messages: Vec::new(),
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

    fn add_id_to_format_message(&mut self, call_expr: &CallExpr) {
        if !self.is_format_message_call(call_expr) {
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
        if !self.is_define_messages_call(call_expr) {
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
