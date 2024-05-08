mod mark_for_deletion;
pub mod track_files_for_deletion;

use std::fs::remove_file;
use std::io;

use crate::configs::config::PathConfig;
use crate::logging::process_directory_tree::FileSystemStack;
use crate::logging::TextOverviewType;
use track_files_for_deletion::track_files_for_deletion;

/// Processes a given configuration to track files for deletion and generate a deletion overview.
///
/// # Arguments
///
/// * `config` - A reference to a `PathConfig` that specifies the configuration for file deletion.
///
/// # Returns
///
/// * `Ok(String)` - A string representing the deletion overview if the operation is successful.
/// * `Err(std::io::Error)` - An error if tracking files for deletion fails.
pub fn track_files_for_deletion_in_given_config(
    config: &PathConfig,
    deletion_overview: &TextOverviewType,
) -> Result<(String, FileSystemStack), std::io::Error> {
    let tracked_files = track_files_for_deletion(config);

    match tracked_files {
        Ok((file_folder_queue, file_folder_metadata)) => {
            let text_overview = deletion_overview.generate_text(
                config,
                file_folder_metadata,
                &config.display_units,
            );
            return Ok((text_overview, file_folder_queue));
        }
        Err(e) => {
            eprintln!("Error tracking files for deletion: {}", e);
            return Err(e);
        }
    }
}

/// Attempts to delete files specified in a `FileSystemStack`.
///
/// Iterates over `DirectoryLeaf` entries in the `files_for_deletion` stack, attempting
/// to delete each. Logs errors for files that cannot be deleted and continues with others.
///
/// # Arguments
/// * `files_for_deletion` - Stack of files scheduled for deletion.
///
/// # Returns
/// Returns `Ok(())` if all files are deleted, or `Err(io::Error)` if any deletions fail.
///
/// # Example
/// ```
/// let mut files = FileSystemStack::new();
/// files.push(DirectoryLeaf { key: PathBuf::from("/path/to/file.txt") });
///
/// match delete_files_scheduled_for_deletion(files) {
///     Ok(()) => println!("All files deleted successfully."),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn delete_files_scheduled_for_deletion(
    files_for_deletion: FileSystemStack,
) -> Result<(), io::Error> {
    let mut errors: bool = false;

    for directory_leaf in files_for_deletion {
        let path = directory_leaf.key;
        if path.is_file() {
            if let Err(e) = remove_file(&path) {
                eprintln!("Failed to delete file {:?}: {}", &path, e);
                errors = true
            }
        }
    }

    match errors {
        true => Err(io::Error::new(
            io::ErrorKind::Other,
            "Warning: not all files could be deleted",
        )),
        false => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::folder_tree_helpers::DirTreeLeaf;
    use std::collections::VecDeque;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    // Helper function to create files
    fn create_files(dir: &Path, file_ext: &str, number_of_files: u32) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for i in 1..=number_of_files {
            let mut file_path = PathBuf::from(dir);
            file_path.push(format!("file{}.{}", i, file_ext));
            paths.push(file_path.clone());
            let mut file = File::create(&file_path).expect("Failed to create file");
            writeln!(file, "This is file number {}", i).expect("Failed to write to file");
        }
        paths
    }
    fn check_number_of_files(path: PathBuf) -> u32 {
        let mut number_of_files = 0;
        if path.is_file() {
            return 1;
        }
        // If it's a directory, recursively count files
        if path.is_dir() {
            for entry in fs::read_dir(path).expect("Failed to read directory") {
                let entry = entry.expect("Failed to get entry");
                let entry_path = entry.path();
                number_of_files += check_number_of_files(entry_path);
            }
        }
        number_of_files
    }

    #[test]
    fn test_delete_files() {
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path();
        let mut all_files = VecDeque::new();

        for path in create_files(base_dir, "txt", 10) {
            all_files.push_back(DirTreeLeaf {
                key: path,
                depth: 1,
                is_last: false,
            });
        }
        let sub_dir = base_dir.join("subdir");
        fs::create_dir(&sub_dir).expect("Failed to create subdirectory");
        for path in create_files(&sub_dir, "txt", 10) {
            all_files.push_back(DirTreeLeaf {
                key: path,
                depth: 2,
                is_last: false,
            });
        }
        assert_eq!(check_number_of_files(base_dir.to_path_buf()), 20);

        let deletion_result = delete_files_scheduled_for_deletion(all_files);
        assert!(deletion_result.is_ok());
        assert_eq!(check_number_of_files(base_dir.to_path_buf()), 0);
    }
}
