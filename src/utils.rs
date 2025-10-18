use crate::types::{PluginState, RemovePrefix, IncludeExportName};
use murmur3::murmur3_32;
use regex::Regex;
use swc_core::ecma::ast::*;
use std::io::Cursor;
use std::collections::HashSet;

pub fn create_hash(message: &str) -> String {
    // Use murmur3 hash with seed 0 to match babel plugin behavior
    let mut cursor = Cursor::new(message.as_bytes());
    murmur3_32(&mut cursor, 0).unwrap_or(0).to_string()
}

pub fn dot_path(str: &str, separator: &str) -> String {
    str.replace(std::path::MAIN_SEPARATOR, separator)
}

pub fn escape_regex(text: &str) -> String {
    regex::escape(text)
}

pub fn dot_path_replace(
    formatted: &str,
    remove_prefix: &str,
    separator: &str,
) -> String {
    let formatted_path = dot_path(formatted, separator);
    
    // Convert remove_prefix to use the same separator as the formatted path
    let normalized_prefix = dot_path(remove_prefix, separator);
    
    // Remove trailing separator from prefix if it exists
    let normalized_prefix = if normalized_prefix.ends_with(separator) {
        &normalized_prefix[..normalized_prefix.len() - separator.len()]
    } else {
        &normalized_prefix
    };
    
    // If the formatted path starts with the normalized prefix, remove it
    if formatted_path.starts_with(normalized_prefix) {
        let remaining = &formatted_path[normalized_prefix.len()..];
        // Remove leading separator if it exists
        if remaining.starts_with(separator) {
            remaining[separator.len()..].to_string()
        } else {
            remaining.to_string()
        }
    } else {
        formatted_path
    }
}

pub fn get_prefix(
    state: &PluginState,
    export_name: Option<&str>,
) -> String {
    let PluginState { filename, opts } = state;
    
    // Handle removePrefix boolean true case
    if let Some(RemovePrefix::Boolean(true)) = opts.remove_prefix {
        return export_name.unwrap_or("").to_string();
    }
    
    // Get the base path from filename
    let mut base_path = filename.to_string_lossy().to_string();
    
    // Remove file extension
    if let Some(ext_pos) = base_path.rfind('.') {
        base_path = base_path[..ext_pos].to_string();
    }
    
    // Convert path separators to dots
    let base_path = dot_path(&base_path, &opts.separator);
    
    // Apply removePrefix options
    let prefix = match &opts.remove_prefix {
        Some(RemovePrefix::Boolean(false)) | None => base_path,
        Some(RemovePrefix::String(s)) => {
            // Check if the string contains regex patterns (like .* or .+)
            if s.contains(".*") || s.contains(".+") || s.contains("[") || s.contains("(") {
                // For regex, we need to work with the original path format, not the dot-separated format
                let original_path = filename.to_string_lossy().to_string();
                let regex = match Regex::new(s) {
                    Ok(r) => r,
                    Err(_) => return export_name.unwrap_or("").to_string(),
                };
                let mut result = regex.replace_all(&original_path, "").to_string();
                
                // Remove file extension if it exists
                if let Some(ext_pos) = result.rfind('.') {
                    result = result[..ext_pos].to_string();
                }
                
                // Convert the result to dot-separated format
                dot_path(&result, &opts.separator)
            } else {
                dot_path_replace(&base_path, s, &opts.separator)
            }
        }
        _ => base_path,
    };
    
    // Apply filebase option
    let prefix = if opts.filebase {
        // Extract just the filename without path
        if let Some(file_name) = filename.file_stem() {
            file_name.to_string_lossy().to_string()
        } else {
            prefix
        }
    } else {
        prefix
    };
    
    // Handle relative_to option
    let prefix = if let Some(relative_to) = &opts.relative_to {
        if prefix.starts_with(relative_to) {
            prefix[relative_to.len()..].trim_start_matches('/').to_string()
        } else {
            prefix
        }
    } else {
        // Auto-detect project root if relative_to is not specified
        if let Some(project_root) = find_project_root(filename) {
            let project_root_str = project_root.to_string_lossy().to_string();
            let project_root_dots = dot_path(&project_root_str, &opts.separator);
            
            if prefix.starts_with(&project_root_dots) {
                prefix[project_root_dots.len()..].trim_start_matches(&opts.separator).to_string()
            } else {
                prefix
            }
        } else {
            prefix
        }
    };
    
    match export_name {
        None => prefix,
        Some(name) => {
            if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}{}{}", prefix, opts.separator, name)
            }
        }
    }
}

pub fn find_project_root(file_path: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut current = file_path.parent()?;
    
    // Look for common project root indicators
    let project_indicators = [
        "yarn.lock",           // Main project indicator (more specific than package.json)
        "package.json",        // Main project indicator
        "package-lock.json",
        "tsconfig.json",
        "babel.config.js",
        "webpack.config.js",
        ".git",
    ];
    
    // Look for project root by checking for indicators
    loop {
        for indicator in &project_indicators {
            if current.join(indicator).exists() {
                return Some(current.to_path_buf());
            }
        }
        
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            break;
        }
    }
    
    None
}

pub fn is_import_local_name(
    name: Option<&str>,
    allowed_names: &[&str],
    _state: &PluginState,
    imported_names: &HashSet<String>,
) -> bool {
    if let Some(name) = name {
        // Check if the specific name is imported and is in the allowed names
        imported_names.contains(name) && allowed_names.iter().any(|&allowed| allowed == name)
    } else {
        // Check if any of the allowed names are imported
        allowed_names.iter().any(|&allowed| imported_names.contains(allowed))
    }
}

pub fn get_leading_comment(_prop: &Prop) -> Option<String> {
    // In SWC, comments are handled differently
    // This is a placeholder implementation
    None
}

pub fn get_export_name(
    call_expr: &CallExpr,
    include_export_name: &Option<IncludeExportName>,
    program: &Program,
) -> Option<String> {
    match include_export_name {
        Some(IncludeExportName::Boolean(true)) => {
            // Look for named export
            find_named_export_name(call_expr, program)
        }
        Some(IncludeExportName::All) => {
            // Look for both named and default export
            find_named_export_name(call_expr, program)
                .or_else(|| find_default_export_name(call_expr, program))
        }
        _ => None,
    }
}

fn find_named_export_name(call_expr: &CallExpr, program: &Program) -> Option<String> {
    // Look for the export declaration that contains this call expression
    match program {
        Program::Module(module) => {
            for item in &module.body {
                match item {
                    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl, .. })) => {
                        match decl {
                            Decl::Var(var_decl) => {
                                for declarator in &var_decl.decls {
                                    if let Some(init) = &declarator.init {
                                        if is_call_expression_in_expr(init, call_expr) {
                                            if let Pat::Ident(ident) = &declarator.name {
                                                return Some(ident.sym.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    None
}

fn is_call_expression_in_expr(expr: &Expr, target_call: &CallExpr) -> bool {
    match expr {
        Expr::Call(call) => {
            // Compare call expressions by their structure
            match (&call.callee, &target_call.callee) {
                (Callee::Expr(callee_expr), Callee::Expr(target_callee_expr)) => {
                    match (callee_expr.as_ref(), target_callee_expr.as_ref()) {
                        (Expr::Ident(callee_ident), Expr::Ident(target_ident)) => {
                            callee_ident.sym == target_ident.sym
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        Expr::Assign(assign) => {
            is_call_expression_in_expr(&assign.right, target_call)
        }
        _ => false,
    }
}

fn find_default_export_name(_call_expr: &CallExpr, _program: &Program) -> Option<String> {
    // This is a simplified implementation
    // In a full implementation, you'd need to traverse the AST to find
    // the default export declaration that contains this call expression
    Some("default".to_string())
}

pub fn object_property(key: &str, value: Expr) -> Prop {
    Prop::KeyValue(KeyValueProp {
        key: PropName::Str(Str {
            span: swc_core::common::DUMMY_SP,
            value: key.into(),
            raw: None,
        }),
        value: Box::new(value),
    })
}

pub fn is_object_properties(properties: &[Prop]) -> bool {
    properties.iter().all(|p| matches!(p, Prop::KeyValue(_)))
}

pub fn get_object_properties(expr: &Expr) -> Option<Vec<PropOrSpread>> {
    match expr {
        Expr::Object(obj) => Some(obj.props.clone()),
        Expr::Ident(_ident) => {
            // This would need to be implemented by looking up the binding
            // in the current scope - simplified for now
            None
        }
        _ => None,
    }
}
