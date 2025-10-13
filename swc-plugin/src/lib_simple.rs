use swc_plugin::{ast::*, plugin_transform, TransformPluginProgramMetadata};
use swc_ecma_ast::*;
use swc_ecma_visit::{as_folder, Fold, VisitMut, VisitMutWith};
use std::path::Path;
use murmur3::murmur3_32;
use std::io::Cursor;

mod utils;

use utils::*;

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<PluginConfig>(
        &metadata.get_transform_plugin_config().unwrap_or("{}".to_string())
    ).unwrap_or_default();

    program.fold_with(&mut as_folder(ReactIntlAutoTransform {
        config,
        filename: metadata.get_context(&metadata.src).unwrap_or_default(),
    }))
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PluginConfig {
    #[serde(default)]
    remove_prefix: Option<String>,
    #[serde(default)]
    filebase: bool,
    #[serde(default = "default_separator")]
    separator: String,
}

fn default_separator() -> String {
    ".".to_string()
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            remove_prefix: None,
            filebase: false,
            separator: ".".to_string(),
        }
    }
}

struct ReactIntlAutoTransform {
    config: PluginConfig,
    filename: String,
}

impl VisitMut for ReactIntlAutoTransform {
    fn visit_mut_jsx_element(&mut self, element: &mut JSXElement) {
        element.visit_mut_children_with(self);
        
        if let JSXElementName::Ident(ident) = &element.opening.name {
            if self.is_react_intl_component(&ident.sym) {
                self.process_jsx_element(element);
            }
        }
    }

    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        call.visit_mut_children_with(self);
        
        if self.is_define_messages_call(call) {
            self.process_define_messages(call);
        }
    }
}

impl ReactIntlAutoTransform {
    fn is_react_intl_component(&self, name: &str) -> bool {
        matches!(name, "FormattedMessage" | "FormattedHTMLMessage")
    }

    fn is_define_messages_call(&self, call: &CallExpr) -> bool {
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                return ident.sym == "defineMessages";
            }
        }
        false
    }

    fn process_jsx_element(&mut self, element: &mut JSXElement) {
        let mut has_id = false;
        let mut default_message_attr: Option<&mut JSXAttr> = None;

        // Find relevant attributes
        for attr in &mut element.opening.attrs {
            if let JSXAttrName::Ident(ident) = &attr.name {
                match ident.sym.as_ref() {
                    "id" => has_id = true,
                    "defaultMessage" => default_message_attr = Some(attr),
                    _ => {}
                }
            }
        }

        // Generate ID if needed
        if !has_id {
            if let Some(default_message_attr) = default_message_attr {
                let id = self.generate_id_for_jsx(default_message_attr);
                if let Some(id) = id {
                    let id_attr = JSXAttr {
                        span: default_message_attr.span,
                        name: JSXAttrName::Ident(Ident::new("id".into(), default_message_attr.span)),
                        value: Some(JSXAttrValue::Lit(Lit::Str(Str {
                            span: default_message_attr.span,
                            value: id.into(),
                            raw: None,
                        }))),
                    };
                    
                    // Insert ID attribute before defaultMessage
                    let mut new_attrs = Vec::new();
                    for attr in &element.opening.attrs {
                        if let JSXAttrName::Ident(ident) = &attr.name {
                            if ident.sym == "defaultMessage" {
                                new_attrs.push(id_attr.clone());
                            }
                        }
                        new_attrs.push(attr.clone());
                    }
                    element.opening.attrs = new_attrs;
                }
            }
        }
    }

    fn process_define_messages(&mut self, call: &mut CallExpr) {
        if let Some(arg) = call.args.first_mut() {
            if let Expr::Object(obj) = &mut *arg.expr {
                self.process_object_expression(obj);
            }
        }
    }

    fn process_object_expression(&mut self, obj: &mut ObjectLit) {
        let prefix = self.get_prefix();
        
        for prop in &mut obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(key_value) = prop.as_ref() {
                    if let PropName::Ident(key_ident) = &key_value.key {
                        let key = key_ident.sym.as_ref();
                        
                        // Process the value
                        if let Expr::Object(value_obj) = &*key_value.value {
                            // Check if ID already exists
                            let has_id = value_obj.props.iter().any(|prop| {
                                if let PropOrSpread::Prop(prop) = prop {
                                    if let Prop::KeyValue(kv) = prop.as_ref() {
                                        if let PropName::Ident(ident) = &kv.key {
                                            return ident.sym == "id";
                                        }
                                    }
                                }
                                false
                            });

                            if !has_id {
                                let id = format!("{}{}{}", prefix, self.config.separator, key);
                                let id_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                    key: PropName::Ident(Ident::new("id".into(), key_value.span)),
                                    value: Box::new(Expr::Lit(Lit::Str(Str {
                                        span: key_value.span,
                                        value: id.into(),
                                        raw: None,
                                    }))),
                                })));
                                
                                // Insert ID property at the beginning
                                let mut new_props = vec![id_prop];
                                new_props.extend(value_obj.props.clone());
                                value_obj.props = new_props;
                            }
                        } else if let Expr::Lit(Lit::Str(str_lit)) = &*key_value.value {
                            // Simple string value - convert to object with id and defaultMessage
                            let id = format!("{}{}{}", prefix, self.config.separator, key);
                            let id_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(Ident::new("id".into(), key_value.span)),
                                value: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: key_value.span,
                                    value: id.into(),
                                    raw: None,
                                }))),
                            })));
                            
                            let default_message_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(Ident::new("defaultMessage".into(), key_value.span)),
                                value: Box::new(Expr::Lit(Lit::Str(str_lit.clone()))),
                            })));
                            
                            *key_value.value = Box::new(Expr::Object(ObjectLit {
                                span: key_value.span,
                                props: vec![id_prop, default_message_prop],
                            }));
                        }
                    }
                }
            }
        }
    }

    fn generate_id_for_jsx(&self, default_message_attr: &JSXAttr) -> Option<String> {
        let message = if let Some(JSXAttrValue::Lit(Lit::Str(str_lit))) = &default_message_attr.value {
            str_lit.value.as_ref()
        } else {
            return None;
        };

        let suffix = create_hash(message);
        Some(self.get_prefix_with_suffix(&suffix))
    }

    fn get_prefix(&self) -> String {
        self.get_prefix_with_suffix("")
    }

    fn get_prefix_with_suffix(&self, suffix: &str) -> String {
        let file_path = Path::new(&self.filename);
        let relative_path = file_path.to_path_buf();

        let formatted = if self.config.filebase {
            relative_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
        } else {
            relative_path.parent()
                .and_then(|p| p.to_str())
                .unwrap_or("")
        };

        let fixed = if let Some(remove_prefix) = &self.config.remove_prefix {
            if remove_prefix == "true" {
                "".to_string()
            } else {
                formatted.replace(remove_prefix, "")
            }
        } else {
            formatted.to_string()
        };

        if suffix.is_empty() {
            fixed
        } else if fixed.is_empty() {
            suffix.to_string()
        } else {
            format!("{}{}{}", fixed, self.config.separator, suffix)
        }
    }
}
