pub mod config;
pub mod config_error;
pub mod parsing;

use config::{PathConfigMap, SizeConfig};
use config_error::ConfigError;
use directories::UserDirs;
use parsing::load_and_parse_config;
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = ".nuke.toml";

/// Retrieves the file path for a user's configuration file.
///
/// This function constructs the path to a user-specific configuration file
/// by appending the provided `config_filename` to the user's home directory.
/// It leverages the `UserDirs` struct to locate the home directory in a platform-independent manner.
///
/// # Arguments
///
/// * `config_filename` - A string slice that holds the name of the configuration file.
///
/// # Returns
///
/// This function returns a `Result` which is either:
/// - `Ok(PathBuf)` containing the path to the configuration file if successful.
/// - `Err(ConfigError::UserDirNotFound)` if the user's home directory cannot be determined.
///
/// # Examples
///
/// ```
/// let config_path = get_user_config_path("app_config.toml").expect("Failed to get config path");
/// println!("Config path: {:?}", config_path);
/// ```
fn get_user_config_path(config_filename: &str) -> Result<PathBuf, ConfigError> {
    let user_dirs = UserDirs::new().ok_or(ConfigError::UserDirNotFound)?;
    let user_dir = user_dirs.home_dir();
    let config_file_path = user_dir.join(config_filename);

    Ok(config_file_path)
}

pub fn report_user_config_path(config_filename: &str) {
    match get_user_config_path(config_filename) {
        Ok(path) => println!("User config path: {:?}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
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
/// * `config_filename` - A string slice that holds the name of the configuration file.
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
/// let user_config = extract_user_config("app_config.toml")
///     .expect("Failed to extract user config");
/// println!("User config extracted successfully.");
/// ```
pub fn extract_user_config(
    config_filename: &str,
) -> Result<(SizeConfig, Option<PathConfigMap>), ConfigError> {
    let user_config_file_path = get_user_config_path(config_filename)?;
    load_and_parse_config(user_config_file_path)
}
