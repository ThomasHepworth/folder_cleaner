use std::path::Path;

pub fn validate_file_path(path: &str) -> bool {
    Path::new(path).exists()
}
