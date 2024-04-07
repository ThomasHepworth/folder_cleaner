// TODO: Add an additional error message detailing how to update the config file, where appropriate
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    UserDirNotFound,
    ReadError(PathBuf),
    ParseError(PathBuf, Box<dyn std::error::Error>),
    ConfigNotFound(PathBuf),
    FolderMapEmpty(String),
}

impl ConfigError {
    pub fn read_error(path: &PathBuf) -> Self {
        ConfigError::ReadError(path.to_path_buf())
    }

    pub fn parse_error(path: &PathBuf, error: toml::de::Error) -> Self {
        ConfigError::ParseError(path.to_path_buf(), Box::new(error))
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::UserDirNotFound => write!(f, "User directory not found."),
            ConfigError::ReadError(path) => {
                write!(f, "Failed to read config file at '{}'.", path.display())
            }
            ConfigError::ParseError(path, err) => write!(
                f,
                "Failed to parse config file at '{}': {}.",
                path.display(),
                err
            ),
            ConfigError::ConfigNotFound(path) => {
                write!(f, "Configuration not found at path: {}", path.display())
            }
            ConfigError::FolderMapEmpty(subgroup) => {
                write!(
                    f,
                    "The subgroup '{}' was not found in your config file.",
                    subgroup
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}
