use chrono::{DateTime, Utc};
use core::panic;
// TODO: Import a faster hmap implementation
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::ffi::OsString;
use std::fs::{self};
use std::io;
use std::path::PathBuf;

use crate::logging::folder_tree::{process_folder_tree_stack, FileSystemLeaf, FileSystemStack};

#[derive(Clone, Debug)]
pub struct Folder {
    path: PathBuf,
    record_last_updated: DateTime<Utc>,
    subfolders: Vec<OsString>,
    folder_size: u64,
}

impl Folder {
    // TODO: Handle any errors properly...
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

    fn scan_folder_metadata(path: &PathBuf) -> io::Result<(Vec<OsString>, u64)> {
        let mut subfolders = Vec::new();
        let mut total_size = 0u64;

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                subfolders.push(entry.file_name());
            } else {
                // Find the size of a file and add this to the total size
                let metadata = get_metadata_or_panic(&path);
                total_size += metadata.len();
            }
        }

        Ok((subfolders, total_size))
    }

    fn scan_and_update_folder_metadata(&mut self) -> io::Result<()> {
        let (subfolders, total_size) = Folder::scan_folder_metadata(&self.path)?;
        self.subfolders = subfolders;
        self.folder_size = total_size;
        self.record_last_updated = Utc::now();
        Ok(())
    }

    pub fn update_folder_if_outdated(&mut self) -> io::Result<&Self> {
        let metadata = fs::metadata(&self.path)?;

        if let Ok(last_modified) = metadata.modified() {
            let last_modified_utc = DateTime::<Utc>::from(last_modified);

            if self.record_last_updated < last_modified_utc {
                println!(
                    "Folder {} last updated at {}. Rescanning",
                    self.path.display(),
                    last_modified_utc
                );
                self.scan_and_update_folder_metadata()?;
            }
        }

        Ok(self) // Return the current instance
    }

    pub fn get_subfolder_full_paths(&self) -> Vec<String> {
        // Get the full file path of all subfolders
        self.subfolders
            .iter()
            .map(|subfolder_name| self.path.join(subfolder_name))
            .filter_map(|path| path.to_str().map(String::from))
            .sorted() // Sort the paths
            .rev() // Reverse the order
            .collect() // Collect into a Vec<String>
    }
}

// TODO: Move to helper module
// Also, handle this error properly - report it as unknown to the user
fn get_metadata_or_panic(path: &PathBuf) -> fs::Metadata {
    match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error scanning folder: {}", e);
            panic!("Failed to retrieve metadata for {:?}", path);
        }
    }
}

// TODO: Implement `to_folder` for `PathBuf`
pub fn crawl_folders_for_metadata(
    folder_hash_map: &mut HashMap<String, Folder>,
    root_folder: &PathBuf,
    crawl_subfolders: bool,
) -> io::Result<()> {
    check_root_folder_exists(root_folder);

    let folder_path = root_folder.to_string_lossy().to_string();
    let dir_tree_stack = process_folder_stack(folder_hash_map, folder_path, crawl_subfolders)?;

    // print folder tree here...
    let dir_tree = process_folder_tree_stack(dir_tree_stack);
    println!("\n{}", dir_tree);
    Ok(())
}

fn process_folder_stack(
    folder_hash_map: &mut HashMap<String, Folder>,
    root_folder_path: String,
    crawl_subfolders: bool,
) -> io::Result<FileSystemStack> {
    let mut dir_tree_stack: FileSystemStack = VecDeque::new();
    let mut folder_stack: FileSystemStack = VecDeque::new();

    folder_stack.push_front(FileSystemLeaf {
        key: root_folder_path,
        depth: 0,
        is_last: true,
    });

    while let Some(current_leaf) = folder_stack.pop_front() {
        update_folder_hashmap(folder_hash_map, &current_leaf.key)?;

        if let Some(folder) = folder_hash_map.get(&current_leaf.key) {
            if crawl_subfolders {
                folder
                    .get_subfolder_full_paths()
                    .into_iter()
                    .enumerate()
                    .for_each(|(index, subfolder_path)| {
                        folder_stack.push_front(FileSystemLeaf {
                            key: subfolder_path,
                            depth: current_leaf.depth + 1,
                            is_last: index == 0,
                        });
                    });
            }
        }

        // Move current_leaf directly to dir_tree_stack
        dir_tree_stack.push_back(current_leaf);
    }

    Ok(dir_tree_stack)
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
    folder_path_str: &str,
) -> io::Result<()> {
    println!("Updating folder: {}", folder_path_str);

    match folder_hash_map.entry(folder_path_str.to_owned()) {
        std::collections::hash_map::Entry::Occupied(mut mapping) => {
            mapping.get_mut().update_folder_if_outdated()?;
        }
        std::collections::hash_map::Entry::Vacant(mapping) => {
            let new_folder = Folder::new(PathBuf::from(folder_path_str));
            mapping.insert(new_folder);
        }
    }

    Ok(())
}
