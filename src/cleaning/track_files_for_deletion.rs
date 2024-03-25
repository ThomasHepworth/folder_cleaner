use super::mark_for_deletion::should_delete_file;
use crate::configs::config::PathConfig;
use crate::logging::folder_tree_helpers::{DirTreeLeaf, TreeKey};
use crate::utils::check_root_folder_exists;
use std::collections::VecDeque;
use std::fs::{self, metadata};
use std::io::Result as IoResult;
use std::path::PathBuf;
use std::time::SystemTime;

type PathVec = Vec<PathBuf>;
type TreeQueue = VecDeque<DirTreeLeaf>;

pub struct DeletionMetaData {
    pub folder_size: u64,
    pub deletion_size: u64,
    pub file_count: usize,
    pub dir_count: usize,
    pub last_modified_time: SystemTime,
}

impl DeletionMetaData {
    pub fn from_root_folder(folder_path: &PathBuf) -> DeletionMetaData {
        let last_modified_time = match metadata(folder_path) {
            Ok(metadata) => metadata.modified().unwrap_or_else(|_| SystemTime::now()),
            Err(_) => SystemTime::now(),
        };

        DeletionMetaData {
            folder_size: 0,
            deletion_size: 0,
            file_count: 0,
            dir_count: 0,
            last_modified_time,
        }
    }
}

pub fn track_files_for_deletion(config: &PathConfig) -> IoResult<(TreeQueue, DeletionMetaData)> {
    let root_folder = config.directory.to_string_lossy().to_string();
    check_root_folder_exists(&root_folder);

    find_deletion_targets(config)
}

fn find_deletion_targets(config: &PathConfig) -> IoResult<(TreeQueue, DeletionMetaData)> {
    // Instantiate our deletion metadata
    let mut deletion_metadata = DeletionMetaData::from_root_folder(&config.directory);
    // TODO: Check whether we can write this using a single VecDeque/Vec
    let mut queue: TreeQueue = VecDeque::new();
    let mut processed_leaves: TreeQueue = VecDeque::new();

    // Add the root folder to the queue
    let root_leaf = DirTreeLeaf::new_root_path(config.directory.clone());
    queue.push_front(root_leaf);

    while let Some(leaf) = queue.pop_front() {
        if leaf.is_file() {
            processed_leaves.push_back(leaf);
            continue;
        }

        let folder_path = leaf.key.as_path();

        match scan_folder_contents(&folder_path, config, &mut deletion_metadata) {
            Ok(directory_contents) => {
                let folder_leaves =
                    create_tree_leaves_from_paths(directory_contents, leaf.depth + 1);
                if folder_leaves.len() == 0 {
                    continue;
                }

                processed_leaves.push_back(leaf);
                deletion_metadata.dir_count += 1;

                for elem in folder_leaves.into_iter().rev() {
                    queue.push_front(elem);
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok((processed_leaves, deletion_metadata))
}

fn scan_folder_contents(
    folder_path: &PathBuf,
    config: &PathConfig,
    deletion_metadata: &mut DeletionMetaData,
) -> IoResult<PathVec> {
    let mut directory_contents: PathVec = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            directory_contents.push(entry_path);
        } else if entry_path.is_file() {
            let should_delete = evaluate_file_for_deletion(&entry_path, config, deletion_metadata)?;
            if should_delete {
                directory_contents.push(entry_path);
            }
        }
    }

    directory_contents.sort_unstable();
    Ok(directory_contents)
}

fn evaluate_file_for_deletion(
    path: &PathBuf,
    config: &PathConfig,
    deletion_metadata: &mut DeletionMetaData,
) -> IoResult<bool> {
    let file_metadata = fs::metadata(&path)?;
    deletion_metadata.folder_size += file_metadata.len();

    if should_delete_file(path, config) {
        deletion_metadata.deletion_size += file_metadata.len();
        deletion_metadata.file_count += 1;

        return Ok(true);
    } else {
        return Ok(false);
    }
}

fn create_tree_leaves_from_paths(paths: PathVec, depth: usize) -> TreeQueue {
    let mut path_leaves: TreeQueue = VecDeque::new();
    let paths_len = paths.len();

    for (index, paths) in paths.into_iter().enumerate() {
        let file_leaf = DirTreeLeaf {
            key: TreeKey::PathKey(paths),
            depth,
            is_last: index == paths_len - 1,
        };
        path_leaves.push_back(file_leaf);
    }

    path_leaves
}
