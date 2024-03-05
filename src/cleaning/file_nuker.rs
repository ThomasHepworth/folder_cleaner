use std::fs;
use std::io;
use std::path::Path;

use crate::configs::config::Config;
use crate::configs::config::PathConfig;

fn delete_files_in_directory(config: &PathConfig) -> io::Result<()> {
    let dir = Path::new(&config.directory);

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() && config.walk {
                // Recurse into subdirectories
                delete_files_in_directory(&PathConfig {
                    directory: path.to_str().unwrap().to_string(),
                    extensions_to_del: config.extensions_to_del.clone(),
                    extensions_to_keep: config.extensions_to_keep.clone(),
                    walk: config.walk,
                })?;
            } else if should_delete_file(
                &path,
                &config.extensions_to_del,
                &config.extensions_to_keep,
            ) {
                fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}

fn should_delete_file(
    path: &Path,
    extensions_to_delete: &[String],
    extensions_to_keep: &[String],
) -> bool {
    // Check if the path has an extension and if it matches
    // any of the extensions in the config.
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => {
            // If the extension is in the list to keep, return false
            // This is triggered first in the case of collisions
            if extensions_to_keep.iter().any(|e| e == ext) {
                false
            } else {
                // Otherwise, check if we should delete based on the extensions list
                extensions_to_delete.is_empty() || extensions_to_delete.iter().any(|e| e == ext)
            }
        }
        None => false,
    }
}

pub fn delete_files_using_config(config: &Config) -> io::Result<()> {
    for path_config in &config.directory_config {
        match delete_files_in_directory(path_config) {
            Ok(()) => println!("Files deleted in directory {}", path_config.directory),
            Err(e) => println!(
                "Error deleting files in {}:\n\t{}",
                path_config.directory, e
            ),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn setup_test_environment(files: Vec<(&str, &str)>) -> PathBuf {
        let dir = tempdir().unwrap();
        for (file_name, content) in files {
            let file_path = dir.path().join(file_name);
            let mut file = File::create(file_path).unwrap();
            writeln!(file, "{}", content).unwrap();
        }
        dir.into_path()
    }

    #[test]
    fn test_deletion_logic() {
        // Setup test environment
        let temp_dir = setup_test_environment(vec![
            ("file_to_delete.txt", "delete me"),
            ("file_to_keep.txt", "keep me"),
            ("another_file_to_keep.md", "also keep me"),
        ]);

        // Configurations
        let config = PathConfig {
            directory: temp_dir.to_str().unwrap().to_string(),
            extensions_to_del: vec!["txt".to_string()],
            extensions_to_keep: vec!["md".to_string()],
            walk: false,
        };

        // Perform deletion
        delete_files_in_directory(&config).unwrap();

        // Assertions
        assert!(
            !temp_dir.join("file_to_delete.txt").exists(),
            "file_to_delete.txt should be deleted"
        );
        assert!(
            temp_dir.join("file_to_keep.txt").exists(),
            "file_to_keep.txt should exist"
        );
        assert!(
            temp_dir.join("another_file_to_keep.md").exists(),
            "another_file_to_keep.md should exist"
        );

        // Cleanup
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
