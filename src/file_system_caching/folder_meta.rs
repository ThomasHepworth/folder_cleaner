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

    fn scan_folder_metadata(path: &PathBuf) -> io::Result<(Vec<PathBuf>, u64)> {
        let mut subfolders = Vec::new();
        let mut total_size = 0u64;

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                subfolders.push(path);
            } else {
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

    let mut folder_stack: VecDeque<(String, u16)> = VecDeque::new();
    // Set our starting folder
    let folder_name: String = root_folder.to_string_lossy().to_string();
    folder_stack.push_front((folder_name, 0));

    process_folder_stack(folder_hash_map, &mut folder_stack, crawl_subfolders)
}

fn process_folder_stack(
    folder_hash_map: &mut HashMap<String, Folder>,
    folder_stack: &mut VecDeque<(String, u16)>,
    crawl_subfolders: bool,
) -> io::Result<()> {
    while let Some((folder_name, depth)) = folder_stack.pop_front() {
        println!("Folder: {} has depth: {}", folder_name, depth);
        update_folder_hashmap(folder_hash_map, &folder_name)?;

        // Check for subfolders to add to the stack (if we are crawling)
        if crawl_subfolders {
            if let Some(folder) = folder_hash_map.get(&folder_name) {
                let subfolder_paths = folder.subfolders.clone();
                for subfolder_path in subfolder_paths {
                    let subfolder_name = subfolder_path.to_string_lossy().to_string();
                    folder_stack.push_front((subfolder_name, depth + 1));
                }
            }
        }
    }

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
    folder_path_str: &str,
) -> io::Result<()> {
    match folder_hash_map.entry(folder_path_str.to_string()) {
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
