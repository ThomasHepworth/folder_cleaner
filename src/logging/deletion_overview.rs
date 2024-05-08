use std::time::SystemTime;

use crate::{
    cleaning::track_files_for_deletion::DeletionMetaData,
    configs::config::{DataSizeUnit, PathConfig},
};
use chrono::{DateTime, Local};

const DASHED_LINE: &str = "---------------------------------------------------------";
const LINE: &str = "=========================================================";

fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}

fn deletion_overview_text() -> Vec<String> {
    [LINE, "📁 Cleaning Overview 📁", DASHED_LINE]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn folder_size_overview_text() -> Vec<String> {
    [LINE, "📁 Folder Size Overview 📁", DASHED_LINE]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn deletion_warning() -> Vec<String> {
    [
        DASHED_LINE,
        &bold("🚨 WARNING: This action is irreversible 🚨"),
        "🛑 Ensure you've backed up any important data before proceeding.",
        "🔍 Review the information carefully before proceeding.",
        LINE,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn format_folder_path(config: &PathConfig) -> String {
    format!("{}: {:?}", bold("Folder path"), config.directory.display())
}

fn format_total_size(total_size: u64, unit: &DataSizeUnit) -> String {
    format!(
        "{}: {}",
        bold("Total folder size"),
        unit.display_total_size(total_size)
    )
}

fn format_deletion_size(deletion_metadata: &DeletionMetaData, unit: &DataSizeUnit) -> String {
    let file_directory_count_text = format!(
        "{} files, {} directories",
        deletion_metadata.file_count, deletion_metadata.dir_count
    );
    format!(
        "{}: {} - {}",
        bold("Data scheduled for deletion"),
        file_directory_count_text,
        bold(&unit.display_total_size(deletion_metadata.deletion_size)),
    )
}

fn format_file_folder_counts(deletion_metadata: &DeletionMetaData, unit: &DataSizeUnit) -> String {
    let file_directory_count_text = format!(
        "{} files, {} directories",
        deletion_metadata.file_count, deletion_metadata.dir_count
    );
    format!(
        "{}: {} - {}",
        bold("Total Size of files and directories"),
        file_directory_count_text,
        bold(&unit.display_total_size(deletion_metadata.deletion_size)),
    )
}

fn format_last_modified(last_modified: SystemTime) -> String {
    let last_modified: DateTime<Local> = last_modified.into();

    format!(
        "{}: {}",
        bold("Last modified date"),
        last_modified.format("%Y-%m-%d %H:%M:%S %Z").to_string()
    )
}

fn create_extensions_string(extensions: &Vec<String>) -> String {
    let formatted: Vec<String> = extensions.iter().map(|ext| format!(".{}", ext)).collect();
    format!("({})", formatted.join(", "))
}

fn format_extensions(config: &PathConfig) -> Vec<String> {
    let mut extension_texts = Vec::new();
    if let Some(exts) = &config.extensions_to_delete {
        let extensions = create_extensions_string(exts);
        extension_texts.push(format!(
            "{}: {:?}",
            bold("Extensions marked for deletion"),
            extensions
        ));
    }
    if let Some(exts) = &config.extensions_to_keep {
        let extensions = create_extensions_string(exts);
        extension_texts.push(format!("{}: {:?}", bold("Extensions to keep"), extensions));
    }
    extension_texts
}

pub fn generate_deletion_overview_text(
    config: &PathConfig, // Assume this is the correct reference to PathConfig
    deletion_metadata: DeletionMetaData,
    unit: &DataSizeUnit,
) -> String {
    let mut deletion_overview: Vec<String> = vec![];
    deletion_overview.extend(deletion_overview_text());

    // Log folder metadata
    deletion_overview.push(format_folder_path(config));
    deletion_overview.push(format_total_size(deletion_metadata.folder_size, unit));
    deletion_overview.push(format_deletion_size(&deletion_metadata, unit));
    deletion_overview.push(format_last_modified(deletion_metadata.last_modified_time));
    deletion_overview.extend(format_extensions(config));
    // Generate warning before asking for deletion confirmation
    deletion_overview.extend(deletion_warning());

    // Return w/ newline separated strings
    deletion_overview.join("\n")
}

pub fn generate_size_overview_text(
    config: &PathConfig, // Assume this is the correct reference to PathConfig
    metadata: DeletionMetaData,
    unit: &DataSizeUnit,
) -> String {
    let mut size_overview: Vec<String> = vec![];
    size_overview.extend(folder_size_overview_text());

    // Log folder metadata
    size_overview.push(format_folder_path(config));
    size_overview.push(format_file_folder_counts(&metadata, unit));
    size_overview.push(format_last_modified(metadata.last_modified_time));
    size_overview.extend(format_extensions(config));

    // Return w/ newline separated strings
    size_overview.join("\n")
}
