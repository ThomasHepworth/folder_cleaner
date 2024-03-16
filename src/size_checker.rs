use crate::configs::config::{DataSizeUnit, PathConfig, SizeConfig};

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn calculate_total_size(dir: &PathBuf, walk: bool) -> u64 {
    let mut total_size: u64 = 0;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok()) // safely handle potential errors during iteration
        .filter(|e| walk || e.path().parent() == Some(dir.as_path()))
    {
        if entry.file_type().is_file() {
            match fs::metadata(entry.path()) {
                Ok(metadata) => total_size += metadata.len(),
                Err(e) => eprintln!("Failed to read metadata for file {:?}: {}", entry.path(), e),
            }
        }
    }

    total_size
}

pub fn calculate_group_file_size(
    user_group: &Vec<PathConfig>,
    walk: bool,
) -> (Vec<(String, u64)>, u64) {
    let mut total_size: u64 = 0;
    let mut group_sizes: Vec<(String, u64)> = Vec::new();

    for entry in user_group {
        let path = &entry.directory;
        let group_size = calculate_total_size(path, walk);
        group_sizes.push((path.display().to_string(), group_size));
        total_size += group_size;
    }

    (group_sizes, total_size)
}

pub fn calculate_and_report_user_group_file_sizes(
    groups: &HashMap<String, Vec<PathConfig>>,
    size_config: &SizeConfig,
) {
    let unit = &size_config.display;
    let walk = size_config.walk;

    for (user_file_group, file_group_info) in groups {
        let (file_sizes, total_size) = calculate_group_file_size(file_group_info, walk);
        print_group_folder_sizes(&user_file_group, total_size, &file_sizes, unit);
    }
}

/// Reports the total size of a user's folder group and the size of individual files within it.
///
/// This function prints a formatted report to the console, detailing the total size of a
/// specified group folder and listing the sizes of individual files within that folder.
///
/// # Arguments
/// * `group_name` - The name of the group folder being reported.
/// * `total_size` - The total size of all files in the group folder.
/// * `file_sizes` - A slice of tuples, each containing a file path and its size.
/// * `unit` - The unit of measurement for file sizes (e.g. "KB", "MB").
///
/// # Example
/// ```
/// // Values are typically parsed from the user's config file, but for demo purposes:
/// let folder_sizes = vec![("user/folder1/".to_string(), 1024)];
/// report_group_folder_size("my_group", 3072, &file_sizes, DisplayUnit::KB);
/// ```
fn print_group_folder_sizes(
    group_name: &str,
    total_size: u64,
    folder_sizes: &[(String, u64)],
    unit: &DataSizeUnit,
) {
    let title = format!(
        "Total size of group '{}': {}",
        group_name,
        unit.display_total_size(total_size)
    );
    // let dashes = "=".repeat(title.len());
    let dashes = "=".repeat(50);

    println!("\n{}\n{}", dashes, title);
    println!("{}\n", dashes);

    if total_size > 0 {
        for (path, size) in folder_sizes {
            println!("    - {:<50} {}", path, unit.display_total_size(*size));
        }
    } else {
        println!("    This group is empty or the files are very small.");
    }

    println!("\n{}", dashes);
}
