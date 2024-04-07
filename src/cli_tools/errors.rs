use crate::configs::errors::ConfigError;
use std::fmt;

#[derive(Debug)]
pub enum CLIError {
    PathOrConfigError(String),
    ConfigError(ConfigError),
}

impl From<ConfigError> for CLIError {
    fn from(error: ConfigError) -> Self {
        CLIError::ConfigError(error)
    }
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CLIError::PathOrConfigError(ref path) => {
                write!(
                    f,
                    "Your entered key: '{}' is neither a valid path or entry in your \
                    config.",
                    path
                )
            }
            CLIError::ConfigError(error) => write!(f, "{}", error),
        }
    }
}
