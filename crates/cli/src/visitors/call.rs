use react_intl_core::ast::call::{
    analyze_define_messages, analyze_format_message, is_define_messages_call,
    is_format_message_call,
};
use react_intl_core::ast::vars::{VarCollector, VarVisitor};
use react_intl_core::types::{CoreState, TransformedMessageData};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

use crate::visitors::import::ImportVisitor;

/// Visitor for extracting messages from call expressions
///
/// Uses VarVisitor to track variable declarations, similar to how
/// ImportVisitor tracks imports. This allows resolution of variable
/// references in calls like `defineMessages(someVar)`.
pub struct CallExpressionVisitor<'a> {
    state: &'a CoreState,
    pub messages: Vec<TransformedMessageData>,
    import_visitor: &'a ImportVisitor<'a>,
    var_visitor: VarVisitor<'a>,
}

impl<'a> CallExpressionVisitor<'a> {
    pub fn new(state: &'a CoreState, import_visitor: &'a ImportVisitor) -> Self {
        Self {
            state,
            import_visitor,
            var_visitor: VarVisitor::new(state),
            messages: Vec::new(),
        }
    }

    pub fn into_messages(self) -> Vec<TransformedMessageData> {
        self.messages
    }

    fn process_format_message(&mut self, call_expr: &CallExpr) {
        if !is_format_message_call(self.import_visitor, call_expr) {
            return;
        }

        if call_expr.args.is_empty() {
            return;
        }

        let expr = &call_expr.args[0].expr;

        match expr.as_ref() {
            Expr::Object(_) => {
                if let Some((transformed, _)) = analyze_format_message(call_expr, self.state) {
                    self.messages.push(transformed);
                }
            }
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
                    if let Some((transformed, _)) =
                        analyze_format_message(&call_expr_for_analysis, self.state)
                    {
                        self.messages.push(transformed);
                    }
                }
            }
            _ => {}
        }
    }

    fn process_define_message(&mut self, call_expr: &CallExpr) {
        if !is_define_messages_call(self.import_visitor, call_expr) {
            return;
        }

        if call_expr.args.is_empty() {
            return;
        }

        let messages = analyze_define_messages(call_expr, self.state, Some(&self.var_visitor));

        for (_, transformed, _) in messages {
            self.messages.push(transformed);
        }
    }
}

impl<'a> Visit for CallExpressionVisitor<'a> {
    fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
        // Track variable declarations using VarVisitor
        // This allows us to resolve variables in function calls
        self.var_visitor.track_declarator(declarator);
        declarator.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        call_expr.visit_children_with(self);

        self.process_format_message(call_expr);
        self.process_define_message(call_expr);
    }
}
