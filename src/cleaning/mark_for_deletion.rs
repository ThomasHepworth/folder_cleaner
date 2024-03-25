use crate::configs::config::PathConfig;
use crate::utils::is_hidden_file;
use std::path::PathBuf;

// TODO: Add hidden file checks to the config?
pub fn should_delete_file(path: &PathBuf, config: &PathConfig) -> bool {
    if is_hidden_file(path) {
        return false; // Skip hidden files
    }

    // Check if the path has an extension and if it matches
    // any of the extensions in the config.
    let file_ext = match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext,
        None => return false,
    };

    if is_extension_to_keep(file_ext, &config.extensions_to_keep) {
        false
    } else {
        is_extension_to_delete(file_ext, &config.extensions_to_delete)
    }
}

fn is_extension_to_keep(ext: &str, extensions_to_keep: &Option<Vec<String>>) -> bool {
    // True if file is set to be kept
    match extensions_to_keep {
        Some(extensions) => extensions.iter().any(|e| e == ext),
        None => false,
    }
}

fn is_extension_to_delete(ext: &str, extensions_to_delete: &Option<Vec<String>>) -> bool {
    match extensions_to_delete {
        // Check if empty or if the extension is in the list -> True in either instance
        Some(extensions) => extensions.is_empty() || extensions.iter().any(|e| e == ext),
        // true if the user has not explicitly specified any extensions to delete
        None => true,
    }
}
