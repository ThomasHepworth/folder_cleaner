mod mark_for_deletion;
pub mod track_files_for_deletion;

use crate::configs::config::PathConfig;
use crate::logging::deletion_overview::generate_deletion_overview_text;
use crate::logging::folder_tree_helpers::DirTreeOptions;
use crate::logging::process_directory_tree::{process_folder_tree_stack, FileSystemStack};
use track_files_for_deletion::track_files_for_deletion;

// TODO: Adjust DirTreeOptions
/// Processes a given configuration to track files for deletion and generate a deletion overview.
///
/// # Arguments
///
/// * `config` - A reference to a `PathConfig` that specifies the configuration for file deletion.
///
/// # Returns
///
/// * `Ok(String)` - A string representing the deletion overview if the operation is successful.
/// * `Err(std::io::Error)` - An error if tracking files for deletion fails.
pub fn process_config(config: &PathConfig) -> Result<(String, FileSystemStack), std::io::Error> {
    let files_for_deletion = track_files_for_deletion(config);

    match files_for_deletion {
        Ok((deletion_queue, deletion_metadata)) => {
            let deletion_overview =
                generate_deletion_overview_text(config, deletion_metadata, &config.display_units);
            return Ok((deletion_overview, deletion_queue));
        }
        Err(e) => {
            eprintln!("Error tracking files for deletion: {}", e);
            return Err(e);
        }
    }
}

/// Prints a deletion tree for a given file system stack.
///
/// # Arguments
///
/// * `deletion_queue` - A `FileSystemStack` that represents the file system stack for deletion.
pub fn print_deletion_tree(deletion_queue: FileSystemStack) {
    let options = DirTreeOptions::default();
    let deletion_tree = process_folder_tree_stack(deletion_queue, &options);
    println!("{}", deletion_tree);
}
