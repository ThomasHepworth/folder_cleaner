use chrono::{DateTime, Utc};
use core::panic;
// TODO: Import a faster hmap implementation
use std::collections::{HashMap, VecDeque};
use std::fs::{self};
use std::io;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Folder {
    path: PathBuf,
    record_last_updated: DateTime<Utc>,
    subfolders: Vec<PathBuf>,
    folder_size: u64,
}

impl Folder {
    pub fn new(path: PathBuf) -> Folder {
        let (subfolders, total_size) = match Folder::scan_folder_metadata(&path) {
            Ok(result) => result,
            Err(_) => (Vec::new(), 0), // Default values for error case
        };

        Folder {
            path,
            record_last_updated: Utc::now(),
            subfolders,
            folder_size: total_size,
        }
    }

    fn scan_folder_metadata(path: &PathBuf) -> io::Result<(Vec<PathBuf>, u64)> {
        let mut subfolders = Vec::new();
        let mut total_size = 0u64;

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                subfolders.push(path);
            } else {
                if let Ok(metadata) = fs::metadata(&path) {
                    total_size += metadata.len();
                }
            }
        }

        Ok((subfolders, total_size))
    }

    pub fn update_folder(&mut self) -> io::Result<()> {
        let metadata = fs::metadata(&self.path)?;

        if let Ok(last_modified) = metadata.modified() {
            let last_modified_utc = DateTime::<Utc>::from(last_modified);

            if self.record_last_updated < last_modified_utc {
                println!(
                    "Folder {} last updated at {}. Rescanning",
                    self.path.display(),
                    last_modified_utc
                );

                match Folder::scan_folder_metadata(&self.path) {
                    // Update our metadata with the latest information
                    Ok((subfolders, total_size)) => {
                        self.subfolders = subfolders;
                        self.folder_size = total_size;
                        self.record_last_updated = Utc::now();
                    }
                    Err(e) => {
                        panic!("Error scanning folder: {}", e);
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn crawl_folders_for_metadata(
    folder_hash_map: &mut HashMap<String, Folder>,
    root_folder: &PathBuf,
    crawl_subfolders: bool,
) -> io::Result<()> {
    check_root_folder_exists(root_folder);

    let folder_name: String = root_folder.to_string_lossy().to_string();
    update_folder_hashmap(folder_hash_map, &folder_name)?;

    if crawl_subfolders {
        recursively_crawl_subfolders(folder_hash_map, &folder_name)?;
    }

    Ok(())
}

fn recursively_crawl_subfolders(
    folder_hash_map: &mut HashMap<String, Folder>,
    root_folder_name: &String,
) -> io::Result<()> {
    // Recursively crawl any subfolders found in the root folder
    match folder_hash_map.get(root_folder_name) {
        Some(folder) => {
            // Clone the paths to avoid borrowing issues
            let subfolder_paths = folder.subfolders.clone();
            for subfolder_path in subfolder_paths {
                crawl_folders_for_metadata(folder_hash_map, &subfolder_path, true)?;
            }
        }
        None => (), // No subfolders - bottom of our recursion
    };

    Ok(())
}

// TODO: Move to error handling module...
fn check_root_folder_exists(root_folder: &PathBuf) {
    // Check if root_folder exists, panic if it doesn't
    if !root_folder.exists() {
        panic!("Invalid root folder: {:?}", root_folder);
    }
}

fn update_folder_hashmap(
    folder_hash_map: &mut HashMap<String, Folder>,
    folder_path_str: &String,
) -> io::Result<()> {
    // Convert the String to a PathBuf
    let folder_path = PathBuf::from(folder_path_str);

    // Check if the folder exists in the map
    if folder_hash_map.contains_key(folder_path_str) {
        // If it exists, update the folder
        folder_hash_map
            .get_mut(folder_path_str)
            .unwrap()
            .update_folder()?;
    } else {
        // If it doesn't exist, create a new Folder and add it to the map
        let new_folder = Folder::new(folder_path.clone());
        folder_hash_map.insert(folder_path_str.clone(), new_folder);
    }

    Ok(())
}
