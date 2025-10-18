use crate::types::{PluginState, RemovePrefix};
use murmur3::murmur3_32;
use regex::Regex;
use swc_core::ecma::ast::*;
use std::io::Cursor;

pub fn create_hash(message: &str) -> String {
    // Use murmur3 hash with seed 0 to match babel plugin behavior
    let mut cursor = Cursor::new(message.as_bytes());
    murmur3_32(&mut cursor, 0).unwrap_or(0).to_string()
}

pub fn dot_path(str: &str, separator: &str) -> String {
    str.replace(std::path::MAIN_SEPARATOR, separator)
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
    
    // Handle relative_to option first (convert absolute path to relative path)
    let prefix = if let Some(relative_to) = &opts.relative_to {
        // Convert relative_to to absolute path if it's relative
        let relative_to_path = if std::path::Path::new(relative_to).is_absolute() {
            std::path::PathBuf::from(relative_to)
        } else {
            // For relative paths, try to find the project root first
            if let Some(project_root) = find_project_root(filename) {
                project_root.join(relative_to)
            } else {
                // Fallback to current directory
                std::env::current_dir()
                    .unwrap_or_else(|_| std::path::PathBuf::from("."))
                    .join(relative_to)
            }
        };
        
        // Calculate the relative path from relative_to to filename
        let filename_path = if std::path::Path::new(filename).is_absolute() {
            std::path::PathBuf::from(filename)
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join(filename)
        };
        
        if let Ok(relative_path) = filename_path.strip_prefix(&relative_to_path) {
            // Convert the relative path to dot-separated format
            let relative_str = relative_path.to_string_lossy().to_string();
            // Remove file extension if it exists
            let relative_str = if let Some(ext_pos) = relative_str.rfind('.') {
                relative_str[..ext_pos].to_string()
            } else {
                relative_str
            };
            dot_path(&relative_str, &opts.separator)
        } else {
            // If we can't calculate relative path, use the original base_path
            base_path
        }
    } else {
        // Auto-detect project root if relative_to is not specified
        if let Some(project_root) = find_project_root(filename) {
            let project_root_str = project_root.to_string_lossy().to_string();
            let project_root_dots = dot_path(&project_root_str, &opts.separator);
            
            if base_path.starts_with(&project_root_dots) {
                let remaining = &base_path[project_root_dots.len()..];
                // Remove leading separator if it exists
                if remaining.starts_with(&opts.separator) {
                    remaining[opts.separator.len()..].to_string()
                } else {
                    remaining.to_string()
                }
            } else {
                base_path
            }
        } else {
            base_path
        }
    };
    
    // Apply removePrefix options after relative_to processing
    let prefix = match &opts.remove_prefix {
        Some(RemovePrefix::Boolean(true)) => {
            // Remove all prefix, return empty string
            String::new()
        }
        Some(RemovePrefix::Boolean(false)) | None => prefix,
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
                
                // If result is empty or just contains separators, return empty string
                if result.trim().is_empty() || result.chars().all(|c| c == '/' || c == '\\') {
                    return export_name.unwrap_or("").to_string();
                }
                
                // Convert the result to dot-separated format
                dot_path(&result, &opts.separator)
            } else {
                dot_path_replace(&prefix, s, &opts.separator)
            }
        }
    };
    
    // Apply filebase option after relative_to and remove_prefix processing
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
