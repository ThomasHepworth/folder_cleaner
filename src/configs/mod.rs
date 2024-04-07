pub mod config;
pub mod errors;
mod parsing;
pub mod unwrap_config_groups;

use directories::UserDirs;
use errors::ConfigError;
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
pub fn get_user_config_path(config_filename: &str) -> Result<PathBuf, ConfigError> {
    let user_dirs = UserDirs::new().ok_or(ConfigError::UserDirNotFound)?;
    let user_dir = user_dirs.home_dir();
    let config_file_path = user_dir.join(config_filename);

    Ok(config_file_path)
}

pub fn report_user_config_path(config_filename: &str) {
    match get_user_config_path(config_filename) {
        Ok(path) => println!(
            "The path to your current configuration file is: {:?}. \
            You can edit this file to customize your cleaning preferences.",
            path
        ),
        Err(e) => eprintln!("Unable to locate the configuration file. Error: {}", e),
    }
}
