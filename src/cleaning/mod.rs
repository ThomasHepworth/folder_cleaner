mod mark_for_deletion;
pub mod track_files_for_deletion;

use crate::configs::config::PathConfig;
use crate::logging::process_directory_tree::FileSystemStack;
use crate::logging::TextOverviewType;
use track_files_for_deletion::track_files_for_deletion;

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
pub fn process_config(
    config: &PathConfig,
    deletion_overview: &TextOverviewType,
) -> Result<(String, FileSystemStack), std::io::Error> {
    let tracked_files = track_files_for_deletion(config);

    match tracked_files {
        Ok((file_folder_queue, file_folder_metadata)) => {
            let text_overview = deletion_overview.generate_text(
                config,
                file_folder_metadata,
                &config.display_units,
            );
            return Ok((text_overview, file_folder_queue));
        }
        Err(e) => {
            eprintln!("Error tracking files for deletion: {}", e);
            return Err(e);
        }
    }
}
