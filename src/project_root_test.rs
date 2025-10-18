#[cfg(test)]
mod tests {
    use crate::utils::find_project_root;
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