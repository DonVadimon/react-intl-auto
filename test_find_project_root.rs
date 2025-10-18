use std::path::PathBuf;

fn find_project_root(file_path: &std::path::Path) -> Option<std::path::PathBuf> {
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

fn main() {
    println!("Testing find_project_root function...");
    
    // Test 1: Relative path
    let relative_path = PathBuf::from("src/components/App.js");
    println!("Testing relative path: {:?}", relative_path);
    if let Some(project_root) = find_project_root(&relative_path) {
        println!("Found project root: {:?}", project_root);
    } else {
        println!("No project root found for relative path");
    }
    
    // Test 2: Absolute path
    let absolute_path = PathBuf::from("/Users/ryan/mine/swc-plugin-react-intl-auto/src/components/App.js");
    println!("Testing absolute path: {:?}", absolute_path);
    if let Some(project_root) = find_project_root(&absolute_path) {
        println!("Found project root: {:?}", project_root);
    } else {
        println!("No project root found for absolute path");
    }
    
    // Test 3: Current working directory
    let cwd = std::env::current_dir().unwrap();
    println!("Current working directory: {:?}", cwd);
    
    // Test 4: Check if project indicators exist
    for indicator in &["yarn.lock", "package.json", ".git"] {
        let path = cwd.join(indicator);
        println!("Checking {:?}: exists = {}", path, path.exists());
    }
}
