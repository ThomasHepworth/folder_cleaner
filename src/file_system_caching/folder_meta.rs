use chrono::{DateTime, Utc};
use core::panic;
use std::path::PathBuf;
use std::time::SystemTime;
// TODO: Import a faster hmap implementation
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{self};
use std::io;

// TODO: Sort strategies
pub enum SortKey {
    FileName,
    Size,
    LastModified,
}

// Decided upon strategy:
// * Folders: housed as objects in our hashmap. Looked up directly. This is a tradeoff between
// space, time and ensuring consistency.
// * Files: stored as FileSortKeys. Metadata easily at hand and only stored in one location.
// Longer term: Work out if sorting strategies for files is necessary.
#[derive(Clone, Debug, Default)]
pub struct FileSortKeys {
    pub file_name: String,
    pub size: u64,
    pub last_modified: Option<SystemTime>,
    pub extension: Option<String>,
}

impl FileSortKeys {
    pub fn from_path(path: &PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();
        let last_modified = metadata.modified().ok();

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|name| name.to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get file name"))?;

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| ext.to_owned());

        Ok(FileSortKeys {
            file_name,
            size,
            last_modified,
            extension,
        })
    }
}

// TODO: Add in a tree size value - would require tweaks to data structures and logic
#[derive(Clone, Debug, Default)]
pub struct FolderMetadata {
    pub subfolders: Vec<String>,
    pub folder_size: u64,
    pub files: Vec<FileSortKeys>,
    pub file_extensions: HashSet<String>,
}

impl FolderMetadata {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn sort_files(&mut self, sort_key: SortKey) {
        match sort_key {
            SortKey::FileName => {
                self.files.sort_by(|a, b| a.file_name.cmp(&b.file_name));
            }
            SortKey::Size => {
                self.files.sort_by(|a, b| a.size.cmp(&b.size));
            }
            SortKey::LastModified => {
                self.files
                    .sort_by(|a, b| a.last_modified.cmp(&b.last_modified));
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Folder {
    // String and not PathBuf to allow for easier serialisation
    path: String, // TODO: I don't think we need this... it's the key
    record_last_updated: DateTime<Utc>,
    pub metadata: FolderMetadata,
}

impl Folder {
    // TODO: Handle any errors properly...
    pub fn new(path: String) -> Folder {
        let (folder_metadata) =
            Folder::scan_folder_metadata(&path).unwrap_or_else(|_| (FolderMetadata::new()));

        Folder {
            path,
            record_last_updated: Utc::now(),
            metadata: folder_metadata,
        }
    }

    fn scan_folder_metadata(path: &str) -> io::Result<FolderMetadata> {
        let path = PathBuf::from(path);
        let mut folder_metadata = FolderMetadata::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                process_directory_entry(&entry_path, &mut folder_metadata);
            } else if entry_path.is_file() {
                if let Some(file) = process_file_entry(&entry_path)? {
                    folder_metadata.folder_size += file.size;
                    if let Some(ext) = &file.extension {
                        folder_metadata.file_extensions.insert(ext.clone());
                    }
                    folder_metadata.files.push(file);
                }
            }
        }

        // TODO: Sort these in the fs::read_dir step?
        folder_metadata.subfolders.sort();

        Ok(folder_metadata)
    }

    fn scan_and_update_folder_metadata(&mut self) -> io::Result<()> {
        let metadata = Folder::scan_folder_metadata(&self.path)?;
        self.metadata = metadata;
        Ok(())
    }

    pub fn update_folder_if_outdated(&mut self) -> io::Result<&Self> {
        let metadata = fs::metadata(&self.path)?;

        if let Ok(last_modified) = metadata.modified() {
            let last_modified_utc = DateTime::<Utc>::from(last_modified);

            if self.record_last_updated < last_modified_utc {
                println!(
                    "Folder {} last updated at {}. Rescanning",
                    self.path, last_modified_utc
                );
                self.scan_and_update_folder_metadata()?;
            }
        }

        Ok(self) // Return the current instance
    }

    pub fn get_subfolder_full_paths(&self) -> Vec<String> {
        // Get the full file path of all subfolders
        self.metadata
            .subfolders
            .iter()
            // .sorted()
            // .rev()
            .map(|subfolder_name| format!("{}/{}", self.path, subfolder_name))
            .collect()
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

fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn process_directory_entry(path: &PathBuf, folder_metadata: &mut FolderMetadata) {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        folder_metadata.subfolders.push(name.to_owned());
    }
}

fn process_file_entry(path: &PathBuf) -> io::Result<Option<FileSortKeys>> {
    if is_hidden(path) {
        Ok(None) // Skip hidden files
    } else {
        FileSortKeys::from_path(path).map(Some)
    }
}

// TODO: Implement `to_folder` for `PathBuf`
pub fn crawl_folders_for_metadata(
    folder_hash_map: &mut HashMap<String, Folder>,
    root_folder: String,
    crawl_subfolders: bool,
) -> io::Result<()> {
    check_root_folder_exists(&root_folder);

    update_folder_mappings(folder_hash_map, root_folder, crawl_subfolders)?;

    // print folder tree here...
    Ok(())
}

// TODO: Parallelise this
fn update_folder_mappings(
    folder_hash_map: &mut HashMap<String, Folder>,
    root_folder_path: String,
    crawl_subfolders: bool,
) -> io::Result<()> {
    let mut folder_stack: VecDeque<String> = VecDeque::new();
    folder_stack.push_front(root_folder_path);

    while let Some(folder_path) = folder_stack.pop_front() {
        if let Err(e) = update_folder_hashmap(folder_hash_map, &folder_path) {
            eprintln!("Error updating folder {}: {}", folder_path, e);
            continue; // Proceed with the next folder in the queue despite the error.
        }

        if crawl_subfolders {
            if let Some(folder) = folder_hash_map.get(&folder_path) {
                folder
                    .get_subfolder_full_paths()
                    .into_iter()
                    .for_each(|subfolder_path| folder_stack.push_front(subfolder_path));
            }
        }
    }

    Ok(())
}

// TODO: Move to error handling module...
fn check_root_folder_exists(root_folder: &str) {
    let root = PathBuf::from(root_folder);

    // Check if root_folder exists, panic if it doesn't
    if !root.exists() {
        panic!("Invalid root folder: {:?}", root_folder);
    }
}

fn update_folder_hashmap(
    folder_hash_map: &mut HashMap<String, Folder>,
    folder_path_str: &str,
) -> io::Result<()> {
    // println!("Updating folder: {}", folder_path_str);

    match folder_hash_map.entry(folder_path_str.to_owned()) {
        std::collections::hash_map::Entry::Occupied(mut mapping) => {
            mapping.get_mut().update_folder_if_outdated()?;
        }
        std::collections::hash_map::Entry::Vacant(mapping) => {
            let new_folder = Folder::new(folder_path_str.to_owned());
            mapping.insert(new_folder);
        }
    }

    Ok(())
}
