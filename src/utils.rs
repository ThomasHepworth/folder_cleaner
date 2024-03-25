use core::panic;
use std::fs;
use std::path::PathBuf;

pub fn check_root_folder_exists(root_folder: &str) {
    let root = PathBuf::from(root_folder);

    // Check if root_folder exists, panic if it doesn't
    if !root.exists() {
        panic!("Invalid root folder: {:?}", root_folder);
    }
}

pub fn get_metadata_or_panic(path: &PathBuf) -> fs::Metadata {
    match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error scanning folder: {}", e);
            panic!("Failed to retrieve metadata for {:?}", path);
        }
    }
}

pub fn is_hidden_file(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
