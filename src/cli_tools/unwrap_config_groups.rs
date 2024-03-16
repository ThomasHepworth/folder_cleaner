use crate::configs::config::{PathConfig, PathConfigMap, SizeConfig};
use crate::configs::{config_error::ConfigError, extract_user_config, CONFIG_FILE_NAME}; // Import the missing module

fn unwrap_all_subgroups(subgroups: PathConfigMap) -> Vec<PathConfig> {
    subgroups.into_iter().flat_map(|(_, group)| group).collect()
}

// TODO: Does this need to error out if nothing is provided?
fn get_subgroup(subgroups: PathConfigMap, subgroup: &str) -> Option<Vec<PathConfig>> {
    subgroups.into_iter().find_map(
        |(key, group)| {
            if key == subgroup {
                Some(group)
            } else {
                None
            }
        },
    )
}

fn get_config_folder_groups(group: Option<&str>, config: PathConfigMap) -> Vec<PathConfig> {
    match group {
        Some(subgroup_name) if !subgroup_name.is_empty() => {
            get_subgroup(config, subgroup_name).unwrap_or_else(Vec::new)
        }
        _ => unwrap_all_subgroups(config),
    }
}

// TODO: Better naming scheme...
pub fn extract_size_and_folder_from_config_groups(
    group: Option<&str>,
) -> Result<(SizeConfig, Vec<PathConfig>), ConfigError> {
    match extract_user_config(CONFIG_FILE_NAME) {
        Ok((size, Some(config_map))) => {
            let folder_configs = get_config_folder_groups(group, config_map);
            Ok((size, folder_configs))
        }
        Ok((_, None)) => Err(ConfigError::FolderMapEmpty(
            "Config map is missing.".to_string(),
        )),
        Err(e) => Err(ConfigError::ConfigNotFound(format!(
            "Error loading config: {}",
            e
        ))),
    }
}
