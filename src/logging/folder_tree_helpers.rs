use crate::{configs::config::DataSizeUnit, file_system_caching::folder_meta::FolderMetadata};
use std::collections::VecDeque; // Add missing import for the `utils` module

// Allows us to define a suffix for our tree, without giving the associated
// value to print. Should also consider implementing DateUnit
#[derive(Debug)]
pub enum SuffixType {
    Display(DataSizeUnit),
}

// Could potentially amalgamate this & TreeSuffix
// Currently this is the highest level of abstraction for the suffix
// It is used as a way of passing a user's config down
impl SuffixType {
    pub fn get_tree_suffix(&self, metadata: &FolderMetadata) -> TreeSuffix {
        match self {
            // TODO: Borrow display unit...
            SuffixType::Display(display_unit) => {
                // TODO: Remove clone?
                TreeSuffix::FileSizeDisplay(display_unit.clone(), metadata.folder_size)
            }
        }
    }
}

// TODO: Come up with stack creation strategies
pub enum TreeSuffix {
    FileSizeDisplay(DataSizeUnit, u64),
}

impl TreeSuffix {
    pub fn get_suffix_str(self) -> String {
        match self {
            TreeSuffix::FileSizeDisplay(display_unit, size) => {
                format!(" - \x1b[1m{}\x1b[0m", display_unit.display_total_size(size),)
            }
        }
    }
}

pub struct DirTreeOptions {
    pub max_depth: Option<usize>,
    pub display_file_size: bool,
    pub display_files: bool,
    pub suffix_type: Option<SuffixType>,
}

impl DirTreeOptions {
    pub fn generate_tree_suffix(&self, metadata: &FolderMetadata) -> Option<TreeSuffix> {
        self.suffix_type.as_ref().and_then(|s| {
            if self.display_file_size {
                // Assuming `get_tree_suffix` can work with a reference to `SuffixType`
                Some(s.get_tree_suffix(&metadata))
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
pub struct DirTreeLeaf {
    pub key: String,
    pub depth: usize,
    pub is_last: bool,
}

impl DirTreeLeaf {
    pub fn new_root(root_folder: String) -> DirTreeLeaf {
        DirTreeLeaf {
            key: root_folder,
            depth: 0,
            is_last: true,
        }
    }
}

pub type FileSystemStack = VecDeque<DirTreeLeaf>;
pub type FileSystemStackWithPath = VecDeque<(String, DirTreeLeaf)>;
