// 临时调试文件
use std::path::Path;

fn dot_path(str: &str, separator: &str) -> String {
    str.replace(std::path::MAIN_SEPARATOR, separator)
}

fn main() {
    let filename = "src/components/App.js";
    let relative_to = "src";
    let separator = ".";
    
    // 模拟 get_prefix 的逻辑
    let mut base_path = filename.to_string();
    
    // Remove file extension
    if let Some(ext_pos) = base_path.rfind('.') {
        base_path = base_path[..ext_pos].to_string();
    }
    
    println!("Base path after removing extension: {}", base_path);
    
    // Convert path separators to dots
    let base_path = dot_path(&base_path, separator);
    println!("Base path after dot_path: {}", base_path);
    
    // Convert relative_to to dots
    let relative_to_dots = dot_path(relative_to, separator);
    println!("Relative_to after dot_path: {}", relative_to_dots);
    
    if base_path.starts_with(&relative_to_dots) {
        let remaining = &base_path[relative_to_dots.len()..];
        println!("Remaining after removing relative_to: {}", remaining);
        
        if remaining.starts_with(separator) {
            let final_result = &remaining[separator.len()..];
            println!("Final result after removing leading separator: {}", final_result);
        } else {
            println!("Final result (no leading separator to remove): {}", remaining);
        }
    } else {
        println!("Base path does not start with relative_to");
    }
}
