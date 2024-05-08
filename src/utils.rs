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

pub fn format_size(bytes: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
    let mut bytes = bytes as f64;

    for unit in units.iter() {
        if bytes < 1024.0 {
            return format!("{:.2} {}", bytes, unit);
        }
        bytes /= 1024.0;
    }

    format!("{:.2} {}", bytes, units.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_various() {
        let test_cases = vec![
            (500, "500.00 B"),
            (1024, "1.00 KiB"),
            (1536, "1.50 KiB"),
            (1048576, "1.00 MiB"),
            (1572864, "1.50 MiB"),
            (1073741824, "1.00 GiB"),
            (1610612736, "1.50 GiB"),
            (1099511627776, "1.00 TiB"),
            (1649267441664, "1.50 TiB"),
            (1125899906842624, "1.00 PiB"),
            (1152921504606846976, "1.00 EiB"),
        ];

        for (bytes, expected) in test_cases {
            assert_eq!(format_size(bytes), expected, "Failed at {} bytes", bytes);
        }
    }
}
