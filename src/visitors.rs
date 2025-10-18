use crate::types::{PluginState, IncludeExportName};
use crate::utils::*;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use std::collections::HashSet;

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
            let name_str = name.sym.to_string();
            let component_name = self.alias_map.get(&name_str)
                .unwrap_or(&name_str);
            
            if REACT_COMPONENTS.contains(&component_name.as_str()) {
                self.process_jsx_element(element, &self.imported_names);
            }
        }
    }
}

impl JSXVisitor {
    fn process_jsx_element(&self, element: &mut JSXElement, _imported_names: &HashSet<String>) {
        let (id_attr, default_message_attr, key_attr) = self.get_element_attributes(&element.opening);
        
        if id_attr.is_none() && default_message_attr.is_some() {
            if let Some(default_message) = default_message_attr {
                self.generate_id(element, default_message, key_attr);
            }
        }
    }
    
    fn get_element_attributes(&self, opening: &JSXOpeningElement) -> (Option<usize>, Option<usize>, Option<usize>) {
        let mut id_idx = None;
        let mut default_message_idx = None;
        let mut key_idx = None;
        
        for (i, attr) in opening.attrs.iter().enumerate() {
            if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
                if let JSXAttrName::Ident(name) = &jsx_attr.name {
                    match name.sym.as_ref() {
                        "id" => id_idx = Some(i),
                        "defaultMessage" => default_message_idx = Some(i),
                        "key" => key_idx = Some(i),
                        _ => {}
                    }
                }
            }
        }
        
        (id_idx, default_message_idx, key_idx)
    }
    
    fn extract_from_value(&self, jsx_attr: &JSXAttr) -> Option<String> {
        match &jsx_attr.value {
            Some(JSXAttrValue::Lit(Lit::Str(str_lit))) => Some(str_lit.value.to_string()),
            Some(JSXAttrValue::JSXExprContainer(JSXExprContainer { expr, .. })) => {
                match expr {
                    JSXExpr::Expr(expr) => {
                        // Try to evaluate the expression statically
                        match expr.as_ref() {
                            Expr::Lit(Lit::Str(str_lit)) => Some(str_lit.value.to_string()),
                            Expr::Tpl(template) => {
                                // Handle template literals
                                if template.exprs.is_empty() {
                                    // Simple template literal without expressions
                                    Some(template.quasis.iter().map(|q| q.raw.to_string()).collect::<String>())
                                } else {
                                    // Template literal with expressions - skip for now
                                    None
                                }
                            }
                            _ => {
                                // For non-analyzable expressions, return None to skip processing
                                return None;
                            }
                        }
                    }
                    _ => {
                        // For non-analyzable expressions, return None to skip processing
                        return None;
                    }
                }
            }
            _ => None,
        }
    }
    
    fn generate_id(&self, element: &mut JSXElement, default_message_idx: usize, key_idx: Option<usize>) {
        let default_message_attr = &element.opening.attrs[default_message_idx];
        if let JSXAttrOrSpread::JSXAttr(jsx_attr) = default_message_attr {
            let suffix = if self.state.opts.use_key {
                if let Some(key_idx) = key_idx {
                    let key_attr = &element.opening.attrs[key_idx];
                    if let JSXAttrOrSpread::JSXAttr(key_jsx_attr) = key_attr {
                        self.extract_from_value(key_jsx_attr)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            
            let suffix = suffix.unwrap_or_else(|| {
                if let Some(message_value) = self.extract_from_value(jsx_attr) {
                    create_hash(&message_value)
                } else {
                    String::new()
                }
            });
            
            let prefix = get_prefix(&self.state, Some(&suffix));
            
            // Insert id attribute before defaultMessage
            let id_attr = JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: swc_core::common::DUMMY_SP,
                name: JSXAttrName::Ident(IdentName::new("id".into(), swc_core::common::DUMMY_SP)),
                value: Some(JSXAttrValue::Lit(Lit::Str(Str {
                    span: swc_core::common::DUMMY_SP,
                    value: prefix.into(),
                    raw: None,
                }))),
            });
            
            element.opening.attrs.insert(default_message_idx, id_attr);
        }
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
        if import_decl.src.value.contains(&self.module_source_name) {
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
                                    let original_name = str_lit.value.to_string();
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
                        self.imported_names.insert(default_spec.local.sym.to_string());
                    }
                    ImportSpecifier::Namespace(namespace_spec) => {
                        self.imported_names.insert(namespace_spec.local.sym.to_string());
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
                    self.variable_declarations.insert(ident.sym.to_string(), obj.clone());
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
        
        // Check if this is a direct call to intl (default import)
        if let Callee::Expr(expr) = &call_expr.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                // Check if this identifier is imported from react-intl
                if self.imported_names.contains(&ident.sym.to_string()) {
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
                self.process_format_message_object(obj);
            }
            Expr::Ident(ident) => {
                // Handle variable reference
                let var_name = ident.sym.to_string();
                if let Some(obj_lit) = self.variable_declarations.get(&var_name).cloned() {
                    let mut obj = obj_lit;
                    self.process_format_message_object(&mut obj);
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
        
        let first_arg = &mut call_expr.args[0];
        let ExprOrSpread { expr, .. } = first_arg;
        if let Expr::Object(obj) = expr.as_mut() {
            // Create a dummy call expression for export name detection
            let dummy_call = CallExpr {
                span: call_expr.span,
                callee: call_expr.callee.clone(),
                args: vec![],
                type_args: call_expr.type_args.clone(),
                ctxt: call_expr.ctxt,
            };
                self.process_define_messages_object(obj, &dummy_call);
        }
    }
    
    fn process_format_message_object(&self, obj: &mut ObjectLit) {
        // Check if id already exists
        let has_id = obj.props.iter().any(|prop| {
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
        
        if has_id {
            return;
        }
        
        // Find defaultMessage property
        let mut default_message_value = None;
        let mut key_value = None;
        
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() {
                    let key_name = match key {
                        PropName::Ident(ident) => Some(ident.sym.as_ref()),
                        PropName::Str(str_lit) => Some(str_lit.value.as_ref()),
                        _ => None,
                    };
                    
                    if let Some(key_str) = key_name {
                        match key_str {
                            "defaultMessage" => {
                                match value.as_ref() {
                                    Expr::Lit(Lit::Str(str_val)) => {
                                        default_message_value = Some(str_val.value.to_string());
                                    }
                                    Expr::Tpl(template) => {
                                        // Handle template literals
                                        if template.exprs.is_empty() {
                                            // Simple template literal without expressions
                                            let message = template.quasis.iter().map(|q| q.raw.to_string()).collect::<String>();
                                            default_message_value = Some(message);
                                        } else {
                                            // Template literal with expressions - convert to string for ID generation
                                            let message = template.quasis.iter().map(|q| q.raw.to_string()).collect::<String>();
                                            default_message_value = Some(message);
                                        }
                                    }
                                    _ => {
                                        // For non-string defaultMessage, convert to string for ID generation
                                        default_message_value = Some(format!("{:?}", value));
                                    }
                                }
                            }
                            "key" => {
                                if let Expr::Lit(Lit::Str(str_val)) = value.as_ref() {
                                    key_value = Some(str_val.value.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        
        if let Some(default_message) = default_message_value {
            let suffix = if self.state.opts.use_key {
                key_value.unwrap_or_else(|| create_hash(&default_message))
            } else {
                create_hash(&default_message)
            };
            
            let id = get_prefix(&self.state, Some(&suffix));
            
            // Add id property
            let id_prop = object_property("id", Expr::Lit(Lit::Str(Str {
                span: swc_core::common::DUMMY_SP,
                value: id.into(),
                raw: None,
            })));
            
            obj.props.push(PropOrSpread::Prop(Box::new(id_prop)));
        }
    }
    
    fn process_define_messages_object(&self, obj: &mut ObjectLit, _call_expr: &CallExpr) {
        // For now, we'll use a simplified approach
        // In a full implementation, we'd need to traverse the AST to find the export declaration
        let export_name = match &self.state.opts.include_export_name {
            Some(IncludeExportName::Boolean(true)) | Some(IncludeExportName::All) => {
                Some("messages".to_string()) // Default export name
            }
            _ => None,
        };
        
        for prop in &mut obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_mut() {
                    match key {
                        PropName::Ident(_) | PropName::Str(_) => {
                            // Get the key name for generating unique IDs
                            let key_name = match key {
                                PropName::Ident(ident) => ident.sym.to_string(),
                                PropName::Str(str_lit) => str_lit.value.to_string(),
                                _ => continue,
                            };
                            
                            match value.as_ref() {
                                Expr::Object(inner_obj) => {
                                    // Check if id already exists
                                    let has_id = inner_obj.props.iter().any(|inner_prop| {
                                        if let PropOrSpread::Prop(inner_prop) = inner_prop {
                                            if let Prop::KeyValue(KeyValueProp { key: inner_key, .. }) = inner_prop.as_ref() {
                                                match inner_key {
                                                    PropName::Ident(ident) => return ident.sym == "id",
                                                    PropName::Str(str_lit) => return str_lit.value == "id",
                                                    _ => {}
                                                }
                                            }
                                        }
                                        false
                                    });
                                    
                                    if !has_id {
                                        // Generate ID with key name and export name if enabled
                                        let id = if let Some(ref export_name) = export_name {
                                            get_prefix(&self.state, Some(&format!("{}.{}", export_name, key_name)))
                                        } else {
                                            get_prefix(&self.state, Some(&key_name))
                                        };
                                        let id_prop = object_property("id", Expr::Lit(Lit::Str(Str {
                                            span: swc_core::common::DUMMY_SP,
                                            value: id.into(),
                                            raw: None,
                                        })));
                                        
                                        if let Expr::Object(inner_obj) = value.as_mut() {
                                            inner_obj.props.push(PropOrSpread::Prop(Box::new(id_prop)));
                                        }
                                    }
                                }
                                Expr::Lit(Lit::Str(str_lit)) => {
                                    // Generate ID with key name and export name if enabled
                                    let id = if let Some(ref export_name) = export_name {
                                        get_prefix(&self.state, Some(&format!("{}.{}", export_name, key_name)))
                                    } else {
                                        get_prefix(&self.state, Some(&key_name))
                                    };
                                    let id_prop = object_property("id", Expr::Lit(Lit::Str(Str {
                                        span: swc_core::common::DUMMY_SP,
                                        value: id.into(),
                                        raw: None,
                                    })));
                                    
                                    let default_message_prop = object_property("defaultMessage", Expr::Lit(Lit::Str(Str {
                                        span: swc_core::common::DUMMY_SP,
                                        value: str_lit.value.clone(),
                                        raw: None,
                                    })));
                                    
                                    *value = Box::new(Expr::Object(ObjectLit {
                                        span: swc_core::common::DUMMY_SP,
                                        props: vec![PropOrSpread::Prop(Box::new(id_prop)), PropOrSpread::Prop(Box::new(default_message_prop))],
                                    }));
                                }
                                _ => {}
                            }
                        }
                        PropName::Num(num_lit) => {
                            // Convert numeric key to string
                            let key_name = num_lit.value.to_string();
                            
                            match value.as_ref() {
                                Expr::Object(inner_obj) => {
                                    // Check if id already exists
                                    let has_id = inner_obj.props.iter().any(|inner_prop| {
                                        if let PropOrSpread::Prop(inner_prop) = inner_prop {
                                            if let Prop::KeyValue(KeyValueProp { key: inner_key, .. }) = inner_prop.as_ref() {
                                                match inner_key {
                                                    PropName::Ident(ident) => return ident.sym == "id",
                                                    PropName::Str(str_lit) => return str_lit.value == "id",
                                                    _ => {}
                                                }
                                            }
                                        }
                                        false
                                    });
                                    
                                    if !has_id {
                                        // Generate ID with numeric key name (as string) and export name if enabled
                                        let id = if let Some(ref export_name) = export_name {
                                            get_prefix(&self.state, Some(&format!("{}.{}", export_name, key_name)))
                                        } else {
                                            get_prefix(&self.state, Some(&key_name))
                                        };
                                        let id_prop = object_property("id", Expr::Lit(Lit::Str(Str {
                                            span: swc_core::common::DUMMY_SP,
                                            value: id.into(),
                                            raw: None,
                                        })));
                                        
                                        if let Expr::Object(inner_obj) = value.as_mut() {
                                            inner_obj.props.push(PropOrSpread::Prop(Box::new(id_prop)));
                                        }
                                    }
                                }
                                Expr::Lit(Lit::Str(str_lit)) => {
                                    // Generate ID with numeric key name (as string) and export name if enabled
                                    let id = if let Some(ref export_name) = export_name {
                                        get_prefix(&self.state, Some(&format!("{}.{}", export_name, key_name)))
                                    } else {
                                        get_prefix(&self.state, Some(&key_name))
                                    };
                                    let id_prop = object_property("id", Expr::Lit(Lit::Str(Str {
                                        span: swc_core::common::DUMMY_SP,
                                        value: id.into(),
                                        raw: None,
                                    })));
                                    
                                    let default_message_prop = object_property("defaultMessage", Expr::Lit(Lit::Str(Str {
                                        span: swc_core::common::DUMMY_SP,
                                        value: str_lit.value.clone(),
                                        raw: None,
                                    })));
                                    
                                    *value = Box::new(Expr::Object(ObjectLit {
                                        span: swc_core::common::DUMMY_SP,
                                        props: vec![PropOrSpread::Prop(Box::new(id_prop)), PropOrSpread::Prop(Box::new(default_message_prop))],
                                    }));
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
