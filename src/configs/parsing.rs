use super::config::{Config, PathConfigMap, SizeConfig};
use super::config_error::ConfigError;

use std::fs;
use std::path::PathBuf;
use toml;
use toml::de::Error;

type ConfigsUnpacked = (SizeConfig, Option<PathConfigMap>);

fn read_config_file(config_file_path: &PathBuf) -> Result<String, ConfigError> {
    fs::read_to_string(config_file_path).map_err(|_| ConfigError::read_error(config_file_path))
}

fn parse_config_from_str(file_content: &str) -> Result<ConfigsUnpacked, Error> {
    let config: Config = toml::from_str(file_content)?;
    Ok((config.size.unwrap_or_default(), config.subgroups))
}

/// Extracts the user configuration from a specified configuration file.
///
/// This function attempts to locate and parse a user-specific configuration file,
/// returning the configuration settings it contains. It first constructs the file path
/// using `get_user_config_path` with the provided `config_filename`, then parses
/// the configuration file into structured data.
///
/// # Arguments
///
/// * `config_file_path` - A PathBuf that holds the full path to the configuration file.
///
/// # Returns
///
/// This function returns a `Result` which is either:
/// - `Ok((SizeConfig, Option<PathConfigMap>))` containing the parsed configuration data if successful.
/// - `Err(ConfigError)` describing the type of error encountered (e.g., file not found, parse error).
///
/// # Examples
///
/// ```
/// let user_config = extract_user_config_from_path("app_config.toml")
///     .expect("Failed to extract user config");
/// ```
pub fn extract_user_config_from_path(
    config_file_path: &PathBuf,
) -> Result<ConfigsUnpacked, ConfigError> {
    let file_content = read_config_file(&config_file_path)
        .map_err(|_| ConfigError::read_error(&config_file_path))?;

    parse_config_from_str(&file_content).map_err(|e| ConfigError::parse_error(&config_file_path, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    // Generic function that creates a temporary TOML file, writes contents to it,
    // and then applies a provided function to the file. It returns the result of that function.
    fn create_temp_toml_file_and_map<F, T>(contents: &str, action: F) -> T
    where
        F: FnOnce(PathBuf) -> T,
    {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.toml");

        let mut file = File::create(&file_path)
            .unwrap_or_else(|e| panic!("Failed to create temporary file: {}", e));

        writeln!(file, "{}", contents)
            .unwrap_or_else(|_| panic!("Failed to write to temporary file"));

        action(file_path.clone())
    }

    #[test]
    fn test_parse_with_subgroups() {
        let toml_str = r#"
            [[downloads]]
            directory = "/example/downloads"
            extensions_to_del = ["tmp", "log", "rs"]

            [[documents]]
            directory = "/example/documents"
            extensions_to_keep = ["pdf", "docx"]
            [[documents]]
            directory = "/example/docs"
            extensions_to_keep = ["xlsx", "pptx"]
        "#;

        let (_, subgroups) = parse_config_from_str(toml_str).unwrap();
        let subgroups = subgroups.unwrap();
        assert_eq!(subgroups.len(), 2);

        // Check the downloads group
        assert_eq!(
            subgroups.get("downloads").unwrap()[0].directory,
            PathBuf::from("/example/downloads")
        );
        assert_eq!(
            subgroups.get("downloads").unwrap()[0]
                .extensions_to_del
                .as_ref()
                .unwrap(),
            &vec!["tmp".to_string(), "log".to_string(), "rs".to_string()]
        );
        assert!(subgroups.get("downloads").unwrap()[0]
            .extensions_to_keep
            .is_none());

        // Check the documents group
        assert_eq!(
            subgroups.get("documents").unwrap()[0]
                .extensions_to_keep
                .as_ref()
                .unwrap(),
            &vec!["pdf".to_string(), "docx".to_string()]
        );
        assert_eq!(
            subgroups.get("documents").unwrap()[1]
                .extensions_to_keep
                .as_ref()
                .unwrap(),
            &vec!["xlsx".to_string(), "pptx".to_string()]
        );
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
    fn test_load_and_parse_invalid_toml() {
        let invalid_toml = r#"
            [[path]
            directory = "/example/path"
        "#; // This TOML string is intentionally invalid

        let result = create_temp_toml_file_and_map(invalid_toml, |file_path| {
            extract_user_config_from_path(&file_path)
        });

        match result {
            Err(ConfigError::ParseError(_, _)) => (), // Test passes, this is expected
            _ => panic!("Test failed, unexpected result: {:#?}", result),
        }
    }

    #[test]
    fn test_config_file_read() {
        let toml_test_str = r#"
            [[path]]
            directory = "/example/path"
        "#;

        let result =
            create_temp_toml_file_and_map(toml_test_str, |file_path| read_config_file(&file_path));

        match result {
            Ok(contents) => {
                assert_eq!(contents.trim(), toml_test_str.trim());
            }
            _ => panic!("Test failed, unexpected result: {:#?}", result),
        }
    }

    // TODO: Extend tests to cover more cases!
    // - Test all of our potential sources of error while deserialising:
    // - Test that the function returns the correct error when the file is not found
    // - Test that the function returns the correct error when the file is not readable
    // - Test that the function returns correctly parsed values in more cases
    // - Validate deserialise errors are handled correctly - e.g. missing fields, wrong types
}
