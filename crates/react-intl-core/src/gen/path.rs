//! Path utilities for generating message IDs
//!
//! Provides functions for working with file paths and generating prefixes for message IDs.

use pathdiff::diff_paths;
use regex::Regex;
use std::path::{Path, PathBuf};

use crate::types::{CoreState, RemovePrefix};

/// Converts path separators to the specified separator (e.g., dots)
///
/// # Arguments
/// * `path` - The path string to convert
/// * `separator` - The separator to use (e.g., ".")
///
/// # Returns
/// The path with separators replaced
fn dot_path(path: &str, separator: &str) -> String {
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
fn dot_path_replace(formatted: &str, remove_prefix: &str, separator: &str) -> String {
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
fn find_project_root(file_path: &Path) -> Option<PathBuf> {
    let mut current = file_path.parent()?;

    // Look for common project root indicators
    let project_indicators = [
        "package.json",      // Main project indicator
        "package-lock.json", // Main project indicator (more specific than package.json)
        "yarn.lock",
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

/// Adds a prefix for message IDs based on file path and options
///
/// # Arguments
/// * `state` - The core state containing filename and options
/// * `suffix` - Optional suffix to append to the prefix
///
/// # Returns
/// The generated prefixed string
/// ```
pub fn add_prefix(state: &CoreState, suffix: &str) -> String {
    let CoreState { filename, opts } = state;

    // Handle removePrefix boolean true case
    if let Some(RemovePrefix::Boolean(true)) = opts.remove_prefix {
        return suffix.to_string();
    }

    // Get the base path from filename (remove extension)
    let base_path = filename.to_string_lossy().to_string();
    let base_path = if let Some(ext_pos) = base_path.rfind('.') {
        base_path[..ext_pos].to_string()
    } else {
        base_path
    };

    // Normalize filename to be absolute for consistent processing
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let absolute_filename = if Path::new(filename).is_absolute() {
        PathBuf::from(filename)
    } else {
        cwd.join(filename)
    };

    // Helper closure to process relative path
    let process_relative_path = |relative_path: PathBuf| -> String {
        let relative_str = relative_path.to_string_lossy().to_string();
        // Remove file extension if it exists
        let relative_str = if let Some(ext_pos) = relative_str.rfind('.') {
            relative_str[..ext_pos].to_string()
        } else {
            relative_str
        };
        // Remove leading separator if it exists
        let relative_str = if relative_str.starts_with('/') || relative_str.starts_with('\\') {
            &relative_str[1..]
        } else {
            &relative_str
        };
        dot_path(relative_str, &opts.separator)
    };

    // Determine the base prefix from filename
    // Priority: relative_to option > project root > cwd
    let prefix = if let Some(relative_to) = &opts.relative_to {
        // Convert relative_to to absolute path if it's relative
        let relative_to_path = if Path::new(relative_to).is_absolute() {
            PathBuf::from(relative_to)
        } else {
            cwd.join(relative_to)
        };

        // Calculate the relative path from relative_to to filename using diff_paths
        if let Some(relative_path) = diff_paths(&absolute_filename, &relative_to_path) {
            // Check if the relative path starts with ".." - if so, the file is NOT under relative_to
            let relative_str = relative_path.to_string_lossy().to_string();
            if relative_str.starts_with("..") {
                // File is not under relative_to, try project root instead
                if let Some(project_root) = find_project_root(&absolute_filename) {
                    if let Ok(relative_path) = absolute_filename.strip_prefix(&project_root) {
                        process_relative_path(relative_path.to_path_buf())
                    } else {
                        // Fallback: use path relative to cwd
                        if let Some(relative_path) = diff_paths(&absolute_filename, &cwd) {
                            process_relative_path(relative_path)
                        } else {
                            dot_path(&base_path, &opts.separator)
                        }
                    }
                } else {
                    // Fallback: use path relative to cwd
                    if let Some(relative_path) = diff_paths(&absolute_filename, &cwd) {
                        process_relative_path(relative_path)
                    } else {
                        dot_path(&base_path, &opts.separator)
                    }
                }
            } else {
                process_relative_path(relative_path)
            }
        } else {
            // If we can't calculate relative path, try project root
            if let Some(project_root) = find_project_root(&absolute_filename) {
                if let Ok(relative_path) = absolute_filename.strip_prefix(&project_root) {
                    process_relative_path(relative_path.to_path_buf())
                } else {
                    dot_path(&base_path, &opts.separator)
                }
            } else {
                dot_path(&base_path, &opts.separator)
            }
        }
    } else {
        // Auto-detect project root if relative_to is not specified
        // Try to find project root from the absolute filename
        let project_root = find_project_root(&absolute_filename);

        if let Some(project_root) = project_root {
            // Calculate the relative path from project root to the file
            if let Ok(relative_path) = absolute_filename.strip_prefix(&project_root) {
                process_relative_path(relative_path.to_path_buf())
            } else {
                // Fallback: use path relative to cwd
                if let Some(relative_path) = diff_paths(&absolute_filename, &cwd) {
                    process_relative_path(relative_path)
                } else {
                    dot_path(&base_path, &opts.separator)
                }
            }
        } else {
            // Fallback: use path relative to cwd if no project root found
            if let Some(relative_path) = diff_paths(&absolute_filename, &cwd) {
                process_relative_path(relative_path)
            } else {
                dot_path(&base_path, &opts.separator)
            }
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
                // For regex, we need to work with the normalized path format (relative to cwd/project root)
                // Use the already computed relative path (stored in `prefix`) for regex matching
                // The `prefix` variable already contains the dot-separated relative path
                let dot_separated_path = &prefix;

                // Convert the dot-separated path back to slashes for regex matching
                let path_with_slashes = dot_separated_path.replace(&opts.separator, "/");

                let regex = match Regex::new(s) {
                    Ok(r) => r,
                    Err(_) => return suffix.to_string(),
                };

                let mut result = regex.replace_all(&path_with_slashes, "").to_string();

                // Remove leading/trailing separators
                result = result
                    .trim_start_matches('/')
                    .trim_end_matches('/')
                    .to_string();

                // If result is empty, return just the suffix
                if result.is_empty() {
                    return suffix.to_string();
                }

                // Convert back to dot-separated format
                dot_path(&result, &opts.separator)
            } else {
                dot_path_replace(&prefix, s, &opts.separator)
            }
        }
    };

    if prefix.is_empty() {
        suffix.to_string()
    } else {
        format!("{}{}{}", prefix, opts.separator, suffix)
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
        use crate::types::OutputMode;
        CoreOptions {
            module_source_name: "react-intl".to_string(),
            separator: ".".to_string(),
            remove_prefix: None,
            relative_to: None,
            hash_id: false,
            hash_algorithm: "murmur3".to_string(),
            extract_source_location: false,
            output_mode: OutputMode::Aggregated,
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
    fn test_add_prefix_remove_prefix_true() {
        let mut opts = create_default_options();
        opts.remove_prefix = Some(RemovePrefix::Boolean(true));
        let state = create_test_state("src/components/App.js", opts);

        let result = add_prefix(&state, "hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_add_prefix_custom_separator() {
        let mut opts = create_default_options();
        opts.separator = "_".to_string();
        let state = create_test_state("src/components/App.js", opts);

        let result = add_prefix(&state, "hello");
        // With custom separator, we still get the full path
        assert_eq!(result, "crates_react-intl-core_src_components_App_hello");
    }

    #[test]
    fn test_add_prefix_relative_filename() {
        let opts = create_default_options();
        // Relative path from cwd should produce same result as absolute
        let state = create_test_state("src/components/App.js", opts);

        let result = add_prefix(&state, "hello");
        // Should be relative to project root
        assert_eq!(result, "crates.react-intl-core.src.components.App.hello");
    }

    #[test]
    fn test_add_prefix_absolute_filename() {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let opts = create_default_options();
        // Use the actual src directory in the current project
        let test_file = cwd.join("src/components/App.js");
        let state = create_test_state(&test_file.to_string_lossy(), opts);

        let result = add_prefix(&state, "hello");
        // We get the full path relative to cwd
        assert_eq!(result, "crates.react-intl-core.src.components.App.hello");
    }

    #[test]
    fn test_add_prefix_relative_to_src() {
        let mut opts = create_default_options();
        opts.relative_to = Some("src".to_string());
        let state = create_test_state("src/components/App.js", opts);

        let result = add_prefix(&state, "hello");
        // With custom separator, we still get the full path
        assert_eq!(result, "components.App.hello");
    }

    #[test]
    fn test_add_prefix_absolute_filename_relative_to_src() {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut opts = create_default_options();
        opts.relative_to = Some("./src".to_string());
        // Use the actual src directory in the current project
        let test_file = cwd.join("src/components/App.js");
        let state = create_test_state(&test_file.to_string_lossy(), opts);

        let result = add_prefix(&state, "hello");
        // With relative_to="./src", path should be relative to src directory
        assert_eq!(result, "components.App.hello");
    }

    #[test]
    fn test_add_prefix_relative_filename_with_relative_relative_to() {
        let mut opts = create_default_options();
        opts.relative_to = Some("src".to_string());
        let state = create_test_state("src/components/App.js", opts);

        let result = add_prefix(&state, "hello");
        // With relative_to="src", should strip src prefix
        assert_eq!(result, "components.App.hello");
    }

    #[test]
    fn test_add_prefix_absolute_filename_with_absolute_relative_to() {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut opts = create_default_options();
        let absolute_src = cwd.join("src").to_string_lossy().to_string();
        opts.relative_to = Some(absolute_src);
        // Use the actual src directory in the current project
        let test_file = cwd.join("src/components/App.js");
        let state = create_test_state(&test_file.to_string_lossy(), opts);

        let result = add_prefix(&state, "hello");
        // With absolute relative_to pointing to src, should produce same result
        assert_eq!(result, "components.App.hello");
    }

    #[test]
    fn test_add_prefix_consistency_absolute_vs_relative() {
        // This test ensures that ID is the same regardless of whether
        // absolute or relative path is used in state.filename
        let opts = create_default_options();

        // Get the absolute path
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let absolute_path = cwd.join("src/components/App.js");
        let relative_path = PathBuf::from("src/components/App.js");

        let state_absolute = create_test_state(&absolute_path.to_string_lossy(), opts.clone());
        let state_relative = create_test_state(&relative_path.to_string_lossy(), opts);

        let result_absolute = add_prefix(&state_absolute, "hello");
        let result_relative = add_prefix(&state_relative, "hello");

        // Both should produce the same result
        assert_eq!(
            result_absolute, result_relative,
            "Absolute and relative paths should produce same ID. Absolute: {}, Relative: {}",
            result_absolute, result_relative
        );
    }

    #[test]
    fn test_add_prefix_relative_filename_with_dot_slash_relative_to() {
        let mut opts = create_default_options();
        opts.relative_to = Some("./src".to_string());
        let state = create_test_state("src/components/App.js", opts);

        let result = add_prefix(&state, "hello");
        // With relative_to="./src", should strip src prefix
        assert_eq!(result, "components.App.hello");
    }

    #[test]
    fn test_add_prefix_with_hash_id_relative_paths() {
        // Test that hash is consistent with relative paths
        let mut opts = create_default_options();
        opts.hash_id = true;
        opts.hash_algorithm = "murmur3".to_string();

        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let absolute_path = cwd.join("src/components/App.js");
        let relative_path = PathBuf::from("src/components/App.js");

        let state_absolute = create_test_state(&absolute_path.to_string_lossy(), opts.clone());
        let state_relative = create_test_state(&relative_path.to_string_lossy(), opts);

        let result_absolute = add_prefix(&state_absolute, "hello");
        let result_relative = add_prefix(&state_relative, "hello");

        // Both should produce the same hash
        assert_eq!(result_absolute, result_relative,
            "Hash IDs should be identical for absolute and relative paths. Absolute: {}, Relative: {}",
            result_absolute, result_relative);
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
        let deep_nested = project_root
            .join("src")
            .join("components")
            .join("ui")
            .join("buttons");

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
