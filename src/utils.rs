use crate::types::{PluginState, RemovePrefix};
use murmur3::murmur3_32;
use regex::Regex;
use std::io::Cursor;
use swc_core::ecma::ast::*;

pub fn create_hash(message: &str) -> String {
    // Use murmur3 hash with seed 0 to match babel plugin behavior
    let mut cursor = Cursor::new(message.as_bytes());
    murmur3_32(&mut cursor, 0).unwrap_or(0).to_string()
}

pub fn hash_string(input: &str, algorithm: &str) -> String {
    match algorithm {
        "base64" => {
            use base64::{engine::general_purpose::STANDARD, Engine as _};
            STANDARD.encode(input.as_bytes())
        }
        "murmur3" | _ => create_hash(input),
    }
}

pub fn dot_path(str: &str, separator: &str) -> String {
    str.replace(std::path::MAIN_SEPARATOR, separator)
}

pub fn dot_path_replace(formatted: &str, remove_prefix: &str, separator: &str) -> String {
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

pub fn get_prefix(state: &PluginState, export_name: Option<&str>) -> String {
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
            // For relative paths, always use current directory as base
            // This matches babel plugin behavior where relative_to is relative to cwd
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join(relative_to)
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
        // Try to find project root from the filename path
        let project_root = if std::path::Path::new(filename).is_absolute() {
            find_project_root(filename)
        } else {
            // For relative paths, try to find project root from current directory
            let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            find_project_root(&cwd)
        };

        if let Some(project_root) = project_root {
            // Calculate the relative path from project root to the file
            let absolute_filename = if std::path::Path::new(filename).is_absolute() {
                std::path::PathBuf::from(filename)
            } else {
                let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                cwd.join(filename)
            };

            if let Ok(relative_path) = absolute_filename.strip_prefix(&project_root) {
                // Convert the relative path to dot-separated format
                let relative_str = relative_path.to_string_lossy().to_string();
                // Remove file extension if it exists
                let relative_str = if let Some(ext_pos) = relative_str.rfind('.') {
                    relative_str[..ext_pos].to_string()
                } else {
                    relative_str
                };
                // Remove leading separator if it exists
                let relative_str =
                    if relative_str.starts_with('/') || relative_str.starts_with('\\') {
                        &relative_str[1..]
                    } else {
                        &relative_str
                    };
                dot_path(relative_str, &opts.separator)
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
        // Keep only directory parts; drop the filename segment from the prefix
        if prefix.is_empty() {
            prefix
        } else {
            let sep = &opts.separator;
            if let Some(pos) = prefix.rfind(sep) {
                prefix[..pos].to_string()
            } else {
                String::new()
            }
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
        "yarn.lock",    // Main project indicator (more specific than package.json)
        "package.json", // Main project indicator
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{PluginOptions, RemovePrefix};
    use std::path::PathBuf;

    fn create_test_state(filename: &str, opts: PluginOptions) -> PluginState {
        PluginState {
            filename: PathBuf::from(filename),
            opts,
        }
    }

    fn create_default_options() -> PluginOptions {
        PluginOptions {
            module_source_name: "react-intl".to_string(),
            separator: ".".to_string(),
            filebase: false,
            remove_prefix: None,
            include_export_name: None,
            use_key: false,
            extract_comments: false,
            relative_to: None,
            hash_id: false,
            hash_algorithm: "murmur3".to_string(),
        }
    }

    #[test]
    fn test_get_prefix_basic() {
        let opts = create_default_options();
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, None);
        assert_eq!(result, "src.components");
    }

    #[test]
    fn test_get_prefix_with_export_name() {
        let opts = create_default_options();
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "src.components.hello");
    }

    #[test]
    fn test_get_prefix_remove_prefix_true() {
        let mut opts = create_default_options();
        opts.remove_prefix = Some(RemovePrefix::Boolean(true));
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_get_prefix_remove_prefix_string() {
        let mut opts = create_default_options();
        opts.remove_prefix = Some(RemovePrefix::String("src/".to_string()));
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "components.hello");
    }

    #[test]
    fn test_get_prefix_remove_prefix_regex() {
        let mut opts = create_default_options();
        opts.remove_prefix = Some(RemovePrefix::String("src/.*/".to_string()));
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_get_prefix_relative_to_relative_path() {
        let mut opts = create_default_options();
        opts.relative_to = Some("src".to_string());
        // Test with relative path - should be relative to current working directory
        // Use relative path for filename to match relative relative_to
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "components.hello");
    }

    #[test]
    fn test_get_prefix_relative_to_absolute_path() {
        let mut opts = create_default_options();
        opts.relative_to = Some("/Users/ryan/project/src".to_string());
        let state = create_test_state("/Users/ryan/project/src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "components.hello");
    }

    #[test]
    fn test_get_prefix_relative_to_nested_relative() {
        let mut opts = create_default_options();
        opts.relative_to = Some("src/components".to_string());
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_get_prefix_relative_to_nested_absolute() {
        let mut opts = create_default_options();
        opts.relative_to = Some("/Users/ryan/project/src/components".to_string());
        let state = create_test_state("/Users/ryan/project/src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_get_prefix_relative_to_with_trailing_slash() {
        let mut opts = create_default_options();
        opts.relative_to = Some("src/".to_string());
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "components.hello");
    }

    #[test]
    fn test_get_prefix_relative_to_no_match() {
        let mut opts = create_default_options();
        opts.relative_to = Some("other".to_string());
        let state = create_test_state("src/components/App.js", opts);

        // When relative_to doesn't match, should return original path
        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "src.components.hello");
    }

    #[test]
    fn test_get_prefix_relative_to_exact_match() {
        let mut opts = create_default_options();
        opts.relative_to = Some("/Users/ryan/project/src/components/App.js".to_string());
        let state = create_test_state("/Users/ryan/project/src/components/App.js", opts);

        // When relative_to matches exactly, should return empty prefix
        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_get_prefix_filebase() {
        let mut opts = create_default_options();
        opts.filebase = true;
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "App.hello");
    }

    #[test]
    fn test_get_prefix_custom_separator() {
        let mut opts = create_default_options();
        opts.separator = "_".to_string();
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "src_components_hello");
    }

    #[test]
    fn test_get_prefix_combined_options() {
        let mut opts = create_default_options();
        opts.relative_to = Some("src".to_string());
        opts.remove_prefix = Some(RemovePrefix::String("components/".to_string()));
        opts.filebase = true;
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "App.hello");
    }

    #[test]
    fn test_get_prefix_empty_export_name() {
        let opts = create_default_options();
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some(""));
        assert_eq!(result, "src.components.");
    }

    #[test]
    fn test_get_prefix_no_export_name() {
        let opts = create_default_options();
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, None);
        assert_eq!(result, "src.components");
    }

    #[test]
    fn test_get_prefix_remove_prefix_boolean_false() {
        let mut opts = create_default_options();
        opts.remove_prefix = Some(RemovePrefix::Boolean(false));
        let state = create_test_state("src/components/App.js", opts);

        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "src.components.hello");
    }

    #[test]
    fn test_get_prefix_with_different_file_extensions() {
        let opts = create_default_options();

        // Test .tsx file
        let state = create_test_state("src/components/App.tsx", opts.clone());
        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "src.components.hello");

        // Test .ts file
        let state = create_test_state("src/components/App.ts", opts.clone());
        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "src.components.hello");

        // Test file without extension
        let state = create_test_state("src/components/App", opts);
        let result = get_prefix(&state, Some("hello"));
        assert_eq!(result, "src.components.hello");
    }
}
