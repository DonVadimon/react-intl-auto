use murmur3::murmur3_32;
use std::io::Cursor;

pub fn create_hash(message: &str) -> String {
    let mut cursor = Cursor::new(message.as_bytes());
    let hash = murmur3_32(&mut cursor, 0).unwrap_or(0);
    hash.to_string()
}

pub fn dot_path(str: &str, separator: &str) -> String {
    str.replace(std::path::MAIN_SEPARATOR, separator)
}
