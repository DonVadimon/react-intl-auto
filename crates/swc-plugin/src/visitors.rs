use std::collections::HashSet;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use react_intl_core::{
    analyze_define_messages, analyze_format_message, analyze_jsx_element, extract_prop_name,
    TransformedMessageData, REACT_COMPONENTS,
};

use react_intl_core::CoreState;

use crate::utils::*;

pub struct JSXVisitor {
    pub state: CoreState,
    pub imported_names: HashSet<String>,
    pub alias_map: std::collections::HashMap<String, String>,
}

impl VisitMut for JSXVisitor {
    fn visit_mut_jsx_element(&mut self, element: &mut JSXElement) {
        element.visit_mut_children_with(self);

        if let JSXElementName::Ident(name) = &element.opening.name {
            // Check if this is a React component, either directly or through alias
            let name_str = name.sym.as_str().to_string();
            let component_name = self.alias_map.get(&name_str).unwrap_or(&name_str);

            // Check if this is a React Intl component and it was imported
            if REACT_COMPONENTS.contains(&component_name.as_str())
                && self.imported_names.contains(&name_str)
            {
                self.process_jsx_element(element);
            }
        }
    }
}

impl JSXVisitor {
    fn process_jsx_element(&mut self, element: &mut JSXElement) {
        // Analyze the JSX element using shared core function
        if let Some((transformed, needs_insertion)) = analyze_jsx_element(element, &self.state) {
            if needs_insertion {
                // ID needs to be inserted - find defaultMessage index and insert ID
                if let Some(default_message_idx) =
                    self.find_attribute_index(&element.opening, "defaultMessage")
                {
                    self.insert_id_attribute(element, default_message_idx, &transformed.id);
                }
            }
            // If needs_insertion is false, ID already exists - do nothing
        }
        // If analyze_jsx_element returns None, this is not a translatable message - do nothing
    }

    fn find_attribute_index(&self, opening: &JSXOpeningElement, name: &str) -> Option<usize> {
        opening.attrs.iter().position(|attr| {
            if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
                if let JSXAttrName::Ident(ident) = &jsx_attr.name {
                    return ident.sym.as_ref() == name;
                }
            }
            false
        })
    }

    fn insert_id_attribute(&self, element: &mut JSXElement, default_message_idx: usize, id: &str) {
        let id_attr = JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: swc_core::common::DUMMY_SP,
            name: JSXAttrName::Ident(IdentName::new("id".into(), swc_core::common::DUMMY_SP)),
            value: Some(JSXAttrValue::Str(Str {
                span: swc_core::common::DUMMY_SP,
                value: id.into(),
                raw: None,
            })),
        });

        element.opening.attrs.insert(default_message_idx, id_attr);
    }
}

pub struct CallExpressionVisitor {
    pub state: CoreState,
    pub imported_names: HashSet<String>,
    pub alias_map: std::collections::HashMap<String, String>,
    pub variable_declarations: std::collections::HashMap<String, ObjectLit>,
}

pub struct ImportVisitor {
    pub imported_names: HashSet<String>,
    pub module_source_name: String,
    pub alias_map: std::collections::HashMap<String, String>,
}

impl VisitMut for ImportVisitor {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        if import_decl.src.value.to_string_lossy() == self.module_source_name {
            for specifier in &import_decl.specifiers {
                match specifier {
                    ImportSpecifier::Named(named_spec) => {
                        if let Some(imported) = &named_spec.imported {
                            match imported {
                                ModuleExportName::Ident(ident) => {
                                    let original_name = ident.sym.to_string();
                                    let local_name = named_spec.local.sym.to_string();

                                    // Store both the imported name and local name
                                    self.imported_names.insert(original_name.clone());
                                    self.imported_names.insert(local_name.clone());

                                    // If there's an alias, create a mapping
                                    if original_name != local_name {
                                        self.alias_map.insert(local_name, original_name);
                                    }
                                }
                                ModuleExportName::Str(str_lit) => {
                                    let original_name = str_lit.value.to_string_lossy().to_string();
                                    let local_name = named_spec.local.sym.to_string();

                                    self.imported_names.insert(original_name.clone());
                                    self.imported_names.insert(local_name.clone());

                                    // If there's an alias, create a mapping
                                    if original_name != local_name {
                                        self.alias_map.insert(local_name, original_name);
                                    }
                                }
                            }
                        } else {
                            // Default import case
                            self.imported_names.insert(named_spec.local.sym.to_string());
                        }
                    }
                    ImportSpecifier::Default(default_spec) => {
                        self.imported_names.insert("default".to_string());
                        self.imported_names
                            .insert(default_spec.local.sym.to_string());
                    }
                    ImportSpecifier::Namespace(namespace_spec) => {
                        self.imported_names
                            .insert(namespace_spec.local.sym.to_string());
                    }
                }
            }
        }
    }
}

impl VisitMut for CallExpressionVisitor {
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

impl CallExpressionVisitor {
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
