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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_path_conditionally_empty_base() {
        let base = "";
        let key = "folder";
        assert_eq!(join_path_conditionally(base, key), "folder");
    }

    #[test]
    fn test_join_path_conditionally_non_empty_base() {
        let base = "/path/to";
        let key = "folder";
        assert_eq!(join_path_conditionally(base, key), "/path/to/folder");
    }

    #[test]
    fn test_join_path_conditionally_root_base() {
        let base = "/";
        let key = "folder";
        assert_eq!(join_path_conditionally(base, key), "/folder");
    }

    #[test]
    fn test_join_path_conditionally_complex_key() {
        let base = "/path/to";
        let key = "nested/folder";
        assert_eq!(join_path_conditionally(base, key), "/path/to/nested/folder");
    }

    #[test]
    fn test_join_path_conditionally_trailing_slash_base() {
        let base = "/path/to/";
        let key = "folder";
        assert_eq!(join_path_conditionally(base, key), "/path/to/folder");
    }
}
