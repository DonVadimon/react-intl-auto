use std::collections::HashSet;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

use react_intl_core::IncludeExportName;
use react_intl_core::{analyze_define_messages, analyze_format_message, analyze_jsx_element};

use crate::types::PluginState;
use crate::utils::*;

const REACT_COMPONENTS: &[&str] = &["FormattedMessage", "FormattedHTMLMessage"];

pub struct JSXVisitor {
    pub state: PluginState,
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

            if REACT_COMPONENTS.contains(&component_name.as_str()) {
                self.process_jsx_element(element, &self.imported_names);
            }
        }
    }
}

impl JSXVisitor {
    fn process_jsx_element(&self, element: &mut JSXElement, _imported_names: &HashSet<String>) {
        // Analyze the JSX element using shared core function
        if let Some((_, transformed, needs_insertion)) = analyze_jsx_element(element, &self.state) {
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
    pub state: PluginState,
    pub imported_names: HashSet<String>,
    pub variable_declarations: std::collections::HashMap<String, ObjectLit>,
}

pub struct ImportVisitor {
    pub imported_names: HashSet<String>,
    pub module_source_name: String,
    pub alias_map: std::collections::HashMap<String, String>,
}

impl VisitMut for ImportVisitor {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        if import_decl
            .src
            .value
            .to_string_lossy()
            .contains(&self.module_source_name)
        {
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
                // Only return true if it's specifically "formatMessage", not other imports like "defineMessages"
                if name == "formatMessage" && self.imported_names.contains(&name) {
                    return true;
                }
            }
        }

        false
    }

    fn is_define_messages_call(&self, call_expr: &CallExpr) -> bool {
        if let Callee::Expr(expr) = &call_expr.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                // Check if this is defineMessages by name
                return ident.sym == "defineMessages";
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

    fn add_id_to_define_message(&self, call_expr: &mut CallExpr) {
        if !self.is_define_messages_call(call_expr) {
            return;
        }

        if call_expr.args.is_empty() {
            return;
        }

        // Get export name for ID generation
        let export_name = match &self.state.opts.include_export_name {
            Some(IncludeExportName::Boolean(true)) | Some(IncludeExportName::All) => {
                Some("messages".to_string()) // Default export name
            }
            _ => None,
        };

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
                self.process_define_messages_object_with_analysis(
                    obj,
                    &call_expr_for_analysis,
                    export_name.as_deref(),
                );
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
                        export_name.as_deref(),
                    );
                    *expr = Box::new(Expr::Object(obj));
                }
            }
            _ => {}
        }
    }

    fn process_format_message_object_with_analysis(
        &self,
        obj: &mut ObjectLit,
        call_expr: &CallExpr,
    ) {
        // Use the shared core function to analyze formatMessage
        if let Some((_, transformed)) = analyze_format_message(call_expr, &self.state) {
            // Check if id already exists
            let has_id = obj.props.iter().any(|prop| {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Prop::KeyValue(KeyValueProp { key, .. }) = prop.as_ref() {
                        match key {
                            PropName::Ident(ident) => ident.sym == "id",
                            PropName::Str(str_lit) => str_lit.value == "id",
                            _ => false,
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            if !has_id {
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
        &self,
        obj: &mut ObjectLit,
        call_expr: &CallExpr,
        export_name: Option<&str>,
    ) {
        // Use the shared core function to analyze defineMessages
        // This returns (key_name, MessageData, TransformedMessageData) for each message
        let messages = analyze_define_messages(call_expr, &self.state, export_name);

        if messages.is_empty() {
            return;
        }

        // Build a map from key name to the transformed ID
        // This uses the ID generated by the shared core crate
        let message_id_map: std::collections::HashMap<String, String> = messages
            .into_iter()
            .map(|(key_name, _message_data, transformed)| (key_name, transformed.id))
            .collect();

        // Update object properties based on analysis
        for prop in &mut obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_mut() {
                    // Extract the key name from the property
                    let key_name = match key {
                        PropName::Ident(ident) => ident.sym.to_string(),
                        PropName::Str(str_lit) => str_lit.value.to_string_lossy().to_string(),
                        PropName::Num(num_lit) => num_lit.value.to_string(),
                        _ => continue,
                    };

                    // Get the pre-generated ID from the shared core analysis
                    let Some(final_id) = message_id_map.get(&key_name) else {
                        continue;
                    };

                    match value.as_ref() {
                        Expr::Object(inner_obj) => {
                            // Check if id already exists in the inner object
                            let has_id = inner_obj.props.iter().any(|inner_prop| {
                                if let PropOrSpread::Prop(inner_prop) = inner_prop {
                                    if let Prop::KeyValue(KeyValueProp { key: inner_key, .. }) =
                                        inner_prop.as_ref()
                                    {
                                        match inner_key {
                                            PropName::Ident(ident) => ident.sym == "id",
                                            PropName::Str(str_lit) => str_lit.value == "id",
                                            _ => false,
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            });

                            if !has_id {
                                let id_prop = object_property(
                                    "id",
                                    Expr::Lit(Lit::Str(Str {
                                        span: swc_core::common::DUMMY_SP,
                                        value: final_id.clone().into(),
                                        raw: None,
                                    })),
                                );

                                if let Expr::Object(inner_obj) = value.as_mut() {
                                    inner_obj.props.push(PropOrSpread::Prop(Box::new(id_prop)));
                                }
                            }
                        }
                        Expr::Lit(Lit::Str(str_lit)) => {
                            // Convert string to object with id and defaultMessage
                            let id_prop = object_property(
                                "id",
                                Expr::Lit(Lit::Str(Str {
                                    span: swc_core::common::DUMMY_SP,
                                    value: final_id.clone().into(),
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
                        Expr::Tpl(template) => {
                            // Convert template literal to object with id and preserved template
                            let id_prop = object_property(
                                "id",
                                Expr::Lit(Lit::Str(Str {
                                    span: swc_core::common::DUMMY_SP,
                                    value: final_id.clone().into(),
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
