use std::path::PathBuf;

/// Joins a base path with a key, handling cases where the base path may be empty.
///
/// # Arguments
///
/// * `path` - A string slice that holds the base path.
/// * `key` - A string slice that holds the key to be appended to the path.
///
/// # Returns
///
/// A `String` that represents the joined path.
pub fn join_path_conditionally(base: &str, key: &str) -> String {
    if base.is_empty() {
        key.to_string()
    } else {
        PathBuf::from(base).join(key).to_string_lossy().into_owned()
    }
}
