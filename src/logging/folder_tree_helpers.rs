use crate::configs::config::DataSizeUnit;
use std::path::PathBuf;

// TODO: Implement 'LastModified' and 'DateCreated' suffixes
#[derive(Debug, Clone)]
pub enum TreeSuffix {
    FileSizeDisplay(DataSizeUnit, u64),
}

impl TreeSuffix {
    pub fn get_suffix_str(&self) -> String {
        match *self {
            TreeSuffix::FileSizeDisplay(ref display_unit, size) => {
                format!(" - \x1b[1m{}\x1b[0m", display_unit.display_total_size(size))
            }
        }
    }
}

pub struct DirTreeOptions {
    pub display_files: bool,
    pub tree_suffix: Option<TreeSuffix>,
}

impl Default for DirTreeOptions {
    fn default() -> Self {
        DirTreeOptions {
            display_files: true,
            tree_suffix: None,
        }
    }
}

impl DirTreeOptions {
    pub fn skip_leaf(&self, path: &PathBuf) -> bool {
        match path.is_file() {
            true => !self.display_files,
            false => false,
        }
    }

    pub fn get_tree_suffix_str(&self) -> String {
        // Default to empty string if no suffix is supplied
        self.tree_suffix
            .as_ref()
            .map_or(String::new(), |suffix| suffix.get_suffix_str())
    }
}

#[derive(Debug, Clone)]
pub enum TreeKey {
    StringKey(String),
    PathKey(PathBuf),
}

impl TreeKey {
    pub fn as_path(&self) -> PathBuf {
        match self {
            TreeKey::StringKey(s) => PathBuf::from(s),
            TreeKey::PathKey(p) => p.clone(),
        }
    }

    pub fn display_key(&self) -> String {
        match self {
            TreeKey::StringKey(s) => s.clone(),
            TreeKey::PathKey(p) => p.file_name().unwrap().to_string_lossy().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirTreeLeaf {
    pub key: TreeKey,
    pub depth: usize,
    pub is_last: bool,
}

impl DirTreeLeaf {
    // Allow dead code as this is used for our cached implementation
    #[allow(dead_code)]
    pub fn new_root_str(root_folder: String) -> DirTreeLeaf {
        DirTreeLeaf {
            key: TreeKey::StringKey(root_folder),
            depth: 0,
            is_last: true,
        }
    }

    pub fn new_root_path(root_folder: PathBuf) -> DirTreeLeaf {
        DirTreeLeaf {
            key: TreeKey::PathKey(root_folder),
            depth: 0,
            is_last: true,
        }
    }

    pub fn is_file(&self) -> bool {
        match &self.key {
            TreeKey::PathKey(p) => p.is_file(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_skip_leaf_with_file_and_display_files_true() {
        let options = DirTreeOptions {
            display_files: true,
            tree_suffix: None,
        };

        let file_path = PathBuf::from("some_file.txt");
        assert!(!options.skip_leaf(&file_path));
    }

    #[test]
    fn test_skip_leaf_with_file_and_display_files_false() {
        let options = DirTreeOptions {
            display_files: false,
            tree_suffix: None,
        };

        let file_path = PathBuf::from("some_file.txt");
        assert!(options.skip_leaf(&file_path));
    }

    #[test]
    fn test_skip_leaf_with_directory_regardless_of_display_files() {
        let options_with_true = DirTreeOptions {
            display_files: true,
            tree_suffix: None,
        };
        let options_with_false = DirTreeOptions {
            display_files: false,
            tree_suffix: None,
        };

        let directory_path = PathBuf::from("some_directory");
        assert!(!options_with_true.skip_leaf(&directory_path));
        assert!(!options_with_false.skip_leaf(&directory_path));
    }

    #[test]
    fn test_get_tree_suffix_str_with_no_suffix() {
        let options = DirTreeOptions {
            display_files: true,
            tree_suffix: None,
        };

        assert_eq!(options.get_tree_suffix_str(), String::new());
    }

    #[test]
    fn test_get_tree_suffix_str_with_suffix() {
        let options = DirTreeOptions {
            display_files: true,
            tree_suffix: Some(TreeSuffix::FileSizeDisplay(DataSizeUnit::MB, 1024)),
        };

        assert_eq!(options.get_tree_suffix_str(), " - 1.00 MB");
    }
}
