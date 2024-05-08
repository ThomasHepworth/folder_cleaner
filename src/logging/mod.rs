mod deletion_overview;
pub mod folder_tree_helpers;
pub mod process_directory_tree;

use crate::{
    cleaning::track_files_for_deletion::DeletionMetaData,
    configs::config::{DataSizeUnit, PathConfig},
};
use deletion_overview::{generate_deletion_overview_text, generate_size_overview_text};
use folder_tree_helpers::DirTreeOptions;
use process_directory_tree::{process_folder_tree_stack, FileSystemStack};

pub enum TextOverviewType {
    Deletion,
    Size,
}

impl TextOverviewType {
    pub fn generate_text(
        &self,
        config: &PathConfig,
        deletion_metadata: DeletionMetaData,
        unit: &DataSizeUnit,
    ) -> String {
        match self {
            TextOverviewType::Deletion => {
                generate_deletion_overview_text(config, deletion_metadata, unit)
            }
            TextOverviewType::Size => generate_size_overview_text(config, deletion_metadata, unit),
        }
    }
}

/// Prints a directory tree for a given file system stack.
///
/// # Arguments
///
/// * `directory_queue` - A `FileSystemStack` that represents the file system stack for either
/// deletion or printing.
pub fn print_directory_tree(directory_queue: FileSystemStack) {
    let options = DirTreeOptions::default();
    let deletion_tree = process_folder_tree_stack(directory_queue, &options);
    println!("{}", deletion_tree);
}
