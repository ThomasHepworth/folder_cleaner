use std::{fmt, path::PathBuf};

use crate::utils::format_size;

pub struct DirTreeOptions {
    pub display_files: bool,
    pub file_size: Option<u64>,
}

impl Default for DirTreeOptions {
    fn default() -> Self {
        DirTreeOptions {
            display_files: true,
            file_size: None,
        }
    }
}

impl DirTreeOptions {
    pub fn should_skip_file_leaf(&self, path: &PathBuf) -> bool {
        // Skip if `display_files` is true and the path is a file
        match path.is_file() {
            // If we don't want to display, we should skip files -> invert
            true => !self.display_files,
            false => false,
        }
    }

    pub fn get_tree_suffix_str(&self) -> String {
        match self.file_size {
            Some(size) => {
                format!(" - \x1b[1m{}\x1b[0m", format_size(size))
            }
            // Default to empty string if no suffix is supplied
            None => String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirTreeLeaf {
    pub key: PathBuf,
    pub depth: usize,
    pub is_last: bool,
}

impl DirTreeLeaf {
    pub fn new_root(root_folder: PathBuf) -> DirTreeLeaf {
        DirTreeLeaf {
            key: root_folder,
            depth: 0,
            is_last: true,
        }
    }
}

impl fmt::Display for DirTreeLeaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.key.to_str() {
            Some(path_str) => write!(f, "{}", path_str),
            None => write!(f, "<invalid utf8 path>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // #[test]
    // fn test_skip_leaf_with_file_and_display_files_true() {
    //     let options = DirTreeOptions {
    //         display_files: true,
    //         tree_suffix: None,
    //     };

    //     let file_path = PathBuf::from("some_file.txt");
    //     assert!(!options.should_skip_file_leaf(&file_path));
    // }

    // #[test]
    // fn test_skip_leaf_with_file_and_display_files_false() {
    //     let mock_fs_ops = MockFileSystemOps {
    //         is_file_response: true,
    //     };
    //     let options = DirTreeOptions {
    //         display_files: false,
    //         tree_suffix: None,
    //         fs_ops: &mock_fs_ops,
    //     };

    //     let file_path = PathBuf::from("some_file.txt");
    //     assert!(options.should_skip_file_leaf(&file_path));
    // }

    #[test]
    fn test_skip_leaf_with_directory_regardless_of_display_files() {
        let options_with_true = DirTreeOptions {
            display_files: true,
            file_size: None,
        };
        let options_with_false = DirTreeOptions {
            display_files: false,
            file_size: None,
        };

        let directory_path = PathBuf::from("some_directory");
        assert!(!options_with_true.should_skip_file_leaf(&directory_path));
        assert!(!options_with_false.should_skip_file_leaf(&directory_path));
    }

    #[test]
    fn test_get_tree_suffix_str_with_no_suffix() {
        let options = DirTreeOptions {
            display_files: true,
            file_size: None,
        };

        assert_eq!(options.get_tree_suffix_str(), String::new());
    }

    #[test]
    fn test_get_tree_suffix_str_with_suffix() {
        let options = DirTreeOptions {
            display_files: true,
            file_size: Some(1024),
        };

        assert_eq!(
            options.get_tree_suffix_str(),
            format!(" - \x1b[1m{}\x1b[0m", "1.00 KB")
        );
    }
}
