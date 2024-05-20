use core::panic;
use std::path::PathBuf;

pub fn check_root_folder_exists(root_folder: &str) {
    let root = PathBuf::from(root_folder);

    // Check if root_folder exists, panic if it doesn't
    if !root.exists() {
        panic!("Invalid root folder: {:?}", root_folder);
    }
}

pub fn is_hidden_file(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn format_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
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
            (1024, "1.00 KB"),
            (1536, "1.50 KB"),
            (1048576, "1.00 MB"),
            (1572864, "1.50 MB"),
            (1073741824, "1.00 GB"),
            (1610612736, "1.50 GB"),
            (1099511627776, "1.00 TB"),
            (1649267441664, "1.50 TB"),
            (1125899906842624, "1.00 PB"),
            (1152921504606846976, "1.00 EB"),
        ];

        for (bytes, expected) in test_cases {
            assert_eq!(format_size(bytes), expected, "Failed at {} bytes", bytes);
        }
    }
}
