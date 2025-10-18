use std::path;

fn main() {
    println!("MAIN_SEPARATOR: '{}'", path::MAIN_SEPARATOR);
    
    let test_path = "src/components/App";
    let result = test_path.replace(path::MAIN_SEPARATOR, ".");
    println!("Input: {}", test_path);
    println!("Output: {}", result);
}
