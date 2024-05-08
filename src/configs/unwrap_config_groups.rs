use crate::configs::config::{PathConfig, PathConfigMap};
use crate::configs::errors::ConfigError;
use crate::configs::parsing::extract_user_config_from_path;

use std::path::PathBuf;

fn unwrap_all_subgroups(subgroups: PathConfigMap) -> Vec<PathConfig> {
    subgroups.into_iter().flat_map(|(_, group)| group).collect()
}

fn get_subgroup(subgroups: PathConfigMap, subgroup: &str) -> Result<Vec<PathConfig>, ConfigError> {
    // Directly consume the hashmap to find the desired subgroup.
    // This avoids cloning by taking ownership of the hashmap and its contents.
    let result = subgroups.into_iter().find(|(key, _)| key == subgroup);

    match result {
        Some((_, group)) => Ok(group),
        None => Err(ConfigError::FolderMapEmpty(subgroup.to_string())),
    }
}

fn get_path_config_folder_groups(
    group: Option<&str>,
    config: PathConfigMap,
) -> Result<Vec<PathConfig>, ConfigError> {
    match group {
        Some(subgroup_name) if !subgroup_name.is_empty() => get_subgroup(config, subgroup_name),
        _ => Ok(unwrap_all_subgroups(config)),
    }
}

pub fn fetch_cli_configs(
    config_filepath: &PathBuf,
    filter_group: Option<&str>,
) -> Result<Vec<PathConfig>, ConfigError> {
    let config_map = extract_user_config_from_path(config_filepath)?;

    let config_map =
        config_map.ok_or_else(|| ConfigError::ConfigNotFound(config_filepath.clone()))?;

    let folder_configs = get_path_config_folder_groups(filter_group, config_map)?;
    Ok(folder_configs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn test_config_fetch_helper(
        filter_group: Option<&str>,
        expected_len: Option<usize>,
    ) -> Result<Vec<PathConfig>, ConfigError> {
        let dummy_contents = r#"
        [[core]]
        directory = "/Users/example/random"
        extensions_to_delete = ["xlsx"]

        [[downloads]]
        directory = "/Users/example/Downloads"
        extensions_to_delete = ["xlsx", ".rs"]
        "#;

        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("dummy_config.toml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", dummy_contents).unwrap();

        let result = fetch_cli_configs(&file_path, filter_group);
        if let Some(expected) = expected_len {
            if let Ok(folder_configs) = &result {
                assert_eq!(
                    folder_configs.len(),
                    expected,
                    "Unexpected number of folder configs"
                );
            }
        }

        result
    }

    #[test]
    fn test_fetch_cli_configs_valid_filter() {
        test_config_fetch_helper(Some("core"), Some(1))
            .expect("Expected valid fetch with 'core' filter");
    }

    #[test]
    fn test_fetch_cli_configs_no_filter() {
        test_config_fetch_helper(None, Some(2)).expect("Expected valid fetch without filter");
    }

    #[test]
    fn test_fetch_cli_configs_invalid_filter() {
        let result = test_config_fetch_helper(Some("nonexistent"), None);
        assert!(
            matches!(result, Err(ConfigError::FolderMapEmpty(_))),
            "Expected error for nonexistent filter group"
        );
    }
}
