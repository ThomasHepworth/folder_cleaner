pub mod config;
pub mod config_error;

use std::fs;
use std::path::PathBuf;
use toml;
use toml::de::Error;

use config::{Config, PathConfigMap, SizeConfig};
use config_error::ConfigError;
use directories::UserDirs;

type ConfigUnpacked = (SizeConfig, Option<PathConfigMap>);

fn read_config_file(config_file_path: &PathBuf) -> Result<String, ConfigError> {
    fs::read_to_string(config_file_path).map_err(|_| ConfigError::read_error(config_file_path))
}

fn parse_config_from_str(file_content: &str) -> Result<ConfigUnpacked, Error> {
    let config: Config = toml::from_str(file_content)?;
    Ok((config.size.unwrap_or_default(), config.subgroups))
}

pub fn load_and_parse_config(config_file_path: PathBuf) -> Result<ConfigUnpacked, ConfigError> {
    let file_content = read_config_file(&config_file_path)
        .map_err(|_| ConfigError::read_error(&config_file_path))?;

    parse_config_from_str(&file_content).map_err(|e| ConfigError::parse_error(&config_file_path, e))
}

fn get_user_config_path(config_filename: &str) -> Result<PathBuf, ConfigError> {
    let user_dirs = UserDirs::new().ok_or(ConfigError::UserDirNotFound)?;
    let user_dir = user_dirs.home_dir();
    let config_file_path = user_dir.join(config_filename);

    Ok(config_file_path)
}

pub fn extract_user_config(
    config_filename: &str,
) -> Result<(SizeConfig, Option<PathConfigMap>), ConfigError> {
    let user_config_file_path = get_user_config_path(config_filename)?;
    load_and_parse_config(user_config_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    // Generic function that creates a temporary TOML file, writes contents to it,
    // and then applies a provided function to the file. It returns the result of that function.
    fn create_temp_toml_file_and_read_or_parse<F, T>(contents: &str, action: F) -> (T, String)
    where
        F: FnOnce(PathBuf) -> T,
    {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.toml");

        let mut file = File::create(&file_path)
            .unwrap_or_else(|e| panic!("Failed to create temporary file: {}", e));

        writeln!(file, "{}", contents)
            .unwrap_or_else(|_| panic!("Failed to write to temporary file"));

        let result = action(file_path.clone());

        (result, contents.to_string())
    }

    #[test]
    fn test_parse_with_subgroups() {
        let toml_str = r#"
            [[downloads]]
            directory = "/example/downloads"
            extensions_to_del = ["tmp", "log"]

            [[documents]]
            directory = "/example/documents"
        "#;

        let (_, subgroups) = parse_config_from_str(toml_str).unwrap();
        let subgroups = subgroups.unwrap();
        assert_eq!(
            subgroups.get("downloads").unwrap()[0].directory,
            PathBuf::from("/example/downloads")
        );
        // TODO: Fix test
        // assert_eq!(
        //     config.subgroups.get("documents").unwrap()[0].extensions_to_del,
        //     vec!["tmp".to_string(), "log".to_string()]
        // );
    }

    #[test]
    fn test_parse_optional_fields() {
        let toml_str = r#"
            [[images]]
            directory = "/example/images"
        "#;

        let (_, subgroups) = parse_config_from_str(toml_str).unwrap();

        // Ensure that subgroups is not None and contains "images"
        assert!(subgroups.is_some(), "Subgroups should be Some");
        let subgroups = subgroups.unwrap();
        assert!(
            subgroups.contains_key("images"),
            "Subgroups should contain 'images'"
        );

        // Retrieve the first PathConfig from the "images" subgroup
        let images_config = &subgroups["images"][0];

        // Perform your assertions
        assert_eq!(
            images_config.directory,
            PathBuf::from("/example/images"),
            "Directory should match"
        );
        assert!(
            images_config.extensions_to_del.is_none(),
            "extensions_to_del should be None"
        );
        assert!(
            images_config.extensions_to_keep.is_none(),
            "extensions_to_keep should be None"
        );
    }

    #[test]
    fn test_parse_fail_due_to_missing_directory() {
        let toml_str = r#"
            [[downloads]]
            directory = "/example/downloads"
            [[documents]]
        "#;

        match parse_config_from_str(toml_str) {
            Err(_) => (), // Test passes if an error is returned
            _ => panic!("Expected a toml::de::Error"),
        }
    }

    #[test]
    fn test_load_and_parse_invalid_toml() {
        let invalid_toml = r#"
            [[path]
            directory = "/example/path"
        "#; // This TOML string is intentionally invalid

        let (result, _) = create_temp_toml_file_and_read_or_parse(invalid_toml, |file_path| {
            load_and_parse_config(file_path)
        });

        match result {
            Err(ConfigError::ParseError(_, _)) => (), // Test passes, this is expected
            _ => panic!("Test failed, unexpected result: {:#?}", result),
        }
    }
}
