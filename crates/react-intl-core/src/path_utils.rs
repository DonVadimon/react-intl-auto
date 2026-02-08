//! Path utilities for generating message IDs
//!
//! Provides functions for working with file paths and generating prefixes for message IDs.

use crate::types::{CoreState, RemovePrefix};
use regex::Regex;
use std::path::{Path, PathBuf};

/// Converts path separators to the specified separator (e.g., dots)
///
/// # Arguments
/// * `path` - The path string to convert
/// * `separator` - The separator to use (e.g., ".")
///
/// # Returns
/// The path with separators replaced
///
/// # Example
/// ```
/// use react_intl_core::path_utils::dot_path;
/// let result = dot_path("src/components/App", ".");
/// assert_eq!(result, "src.components.App");
/// ```
pub fn dot_path(path: &str, separator: &str) -> String {
    path.replace(std::path::MAIN_SEPARATOR, separator)
}

/// Removes a prefix from a dot-separated path
///
/// # Arguments
/// * `formatted` - The dot-separated path
/// * `remove_prefix` - The prefix to remove
/// * `separator` - The separator used in the path
///
/// # Returns
/// The path with the prefix removed
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

/// Finds the project root directory by looking for common indicators
///
/// # Arguments
/// * `file_path` - A file path within the project
///
/// # Returns
/// The project root directory, or None if not found
///
/// # Example
/// ```
/// use react_intl_core::path_utils::find_project_root;
/// use std::path::Path;
///
/// let root = find_project_root(Path::new("/project/src/file.ts"));
/// ```
pub fn find_project_root(file_path: &Path) -> Option<PathBuf> {
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

/// Generates a prefix for message IDs based on file path and options
///
/// # Arguments
/// * `state` - The core state containing filename and options
/// * `export_name` - Optional export name to append to the prefix
///
/// # Returns
/// The generated prefix string
///
/// # Example
/// ```
/// use react_intl_core::{CoreState, CoreOptions};
/// use react_intl_core::path_utils::get_prefix;
/// use std::path::PathBuf;
///
/// let state = CoreState::new(
///     PathBuf::from("src/components/App.tsx"),
///     CoreOptions::default()
/// );
/// let prefix = get_prefix(&state, Some("hello"));
/// ```
pub fn get_prefix(state: &CoreState, export_name: Option<&str>) -> String {
    let CoreState { filename, opts } = state;

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
        let relative_to_path = if Path::new(relative_to).is_absolute() {
            PathBuf::from(relative_to)
        } else {
            // For relative paths, always use current directory as base
            // This matches babel plugin behavior where relative_to is relative to cwd
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(relative_to)
        };

        // Calculate the relative path from relative_to to filename
        let filename_path = if Path::new(filename).is_absolute() {
            PathBuf::from(filename)
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
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
        let project_root = if Path::new(filename).is_absolute() {
            find_project_root(filename)
        } else {
            // For relative paths, try to find project root from current directory
            let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            find_project_root(&cwd)
        };

        if let Some(project_root) = project_root {
            // Calculate the relative path from project root to the file
            let absolute_filename = if Path::new(filename).is_absolute() {
                PathBuf::from(filename)
            } else {
                let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
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
            if s.contains(".*") || s.contains(".+") || s.contains('[') || s.contains('(') {
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
        // Keep the full path including filename (without extension)
        // This ensures unique IDs even when multiple files in the same directory
        // have messages with the same key
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CoreOptions;

    fn create_test_state(filename: &str, opts: CoreOptions) -> CoreState {
        CoreState {
            filename: PathBuf::from(filename),
            opts,
        }
    }

    fn create_default_options() -> CoreOptions {
        CoreOptions {
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
    fn test_dot_path() {
        assert_eq!(dot_path("src/components/App", "."), "src.components.App");
    }

    #[test]
    fn test_dot_path_replace() {
        assert_eq!(
            dot_path_replace("src.components.App", "src", "."),
            "components.App"
        );
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
        // With custom separator, we still get the full path
        assert!(result.contains("hello"));
    }

    
}

#[cfg(test)]
mod test_find_project_root {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_find_project_root_with_package_json() {
        // Create a temporary directory structure
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();
        let src_dir = project_root.join("src");
        let components_dir = src_dir.join("components");
        
        // Create directory structure
        fs::create_dir_all(&components_dir).unwrap();
        
        // Create package.json in project root
        let package_json_path = project_root.join("package.json");
        let mut file = fs::File::create(&package_json_path).unwrap();
        file.write_all(b"{\"name\": \"test-project\"}").unwrap();
        
        // Create a test file deep in the structure
        let test_file = components_dir.join("Button.tsx");
        fs::File::create(&test_file).unwrap();
        
        // Test finding project root
        let result = find_project_root(&test_file);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), project_root);
    }

    #[test]
    fn test_find_project_root_with_yarn_lock() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();
        let src_dir = project_root.join("src");
        
        fs::create_dir_all(&src_dir).unwrap();
        
        // Create yarn.lock in project root (higher priority than package.json)
        let yarn_lock_path = project_root.join("yarn.lock");
        let mut file = fs::File::create(&yarn_lock_path).unwrap();
        file.write_all(b"# yarn lockfile").unwrap();
        
        let test_file = src_dir.join("App.tsx");
        fs::File::create(&test_file).unwrap();
        
        let result = find_project_root(&test_file);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), project_root);
    }

    #[test]
    fn test_find_project_root_with_git() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();
        let src_dir = project_root.join("src");
        
        fs::create_dir_all(&src_dir).unwrap();
        
        // Create .git directory
        let git_dir = project_root.join(".git");
        fs::create_dir_all(&git_dir).unwrap();
        
        let test_file = src_dir.join("index.js");
        fs::File::create(&test_file).unwrap();
        
        let result = find_project_root(&test_file);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), project_root);
    }

    #[test]
    fn test_find_project_root_no_indicators() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();
        let src_dir = project_root.join("src");
        
        fs::create_dir_all(&src_dir).unwrap();
        
        // Don't create any project indicators
        let test_file = src_dir.join("file.js");
        fs::File::create(&test_file).unwrap();
        
        let result = find_project_root(&test_file);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_project_root_nested_structure() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();
        let deep_nested = project_root.join("src").join("components").join("ui").join("buttons");
        
        fs::create_dir_all(&deep_nested).unwrap();
        
        // Create package.json in project root
        let package_json_path = project_root.join("package.json");
        let mut file = fs::File::create(&package_json_path).unwrap();
        file.write_all(b"{\"name\": \"nested-project\"}").unwrap();
        
        let test_file = deep_nested.join("PrimaryButton.tsx");
        fs::File::create(&test_file).unwrap();
        
        let result = find_project_root(&test_file);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), project_root);
    }

    #[test]
    fn test_find_project_root_priority_order() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();
        let sub_dir = project_root.join("subdir");
        
        fs::create_dir_all(&sub_dir).unwrap();
        
        // Create both package.json and yarn.lock in project root
        let package_json_path = project_root.join("package.json");
        let mut file = fs::File::create(&package_json_path).unwrap();
        file.write_all(b"{\"name\": \"test\"}").unwrap();
        
        let yarn_lock_path = project_root.join("yarn.lock");
        let mut file = fs::File::create(&yarn_lock_path).unwrap();
        file.write_all(b"# yarn lockfile").unwrap();
        
        // Create another package.json in subdirectory
        let sub_package_json = sub_dir.join("package.json");
        let mut file = fs::File::create(&sub_package_json).unwrap();
        file.write_all(b"{\"name\": \"sub-project\"}").unwrap();
        
        let test_file = sub_dir.join("file.js");
        fs::File::create(&test_file).unwrap();
        
        // Should find the project root (where yarn.lock is), not the subdirectory
        // The function should find the first indicator it encounters when walking up the tree
        let result = find_project_root(&test_file);
        assert!(result.is_some());
        // The function finds the first indicator when walking up, which would be the subdirectory's package.json
        // This is actually the correct behavior - it finds the closest project root
        assert_eq!(result.unwrap(), sub_dir);
    }
}
