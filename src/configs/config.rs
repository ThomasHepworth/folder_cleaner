use serde::{self, Deserialize, Deserializer};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::canonicalize;
use std::path::PathBuf;

pub type PathConfigMap = HashMap<String, Vec<PathConfig>>;

// TODO: Improve deserialisation of extensions
// Test - extensions_to_del = ["tmp", ".log", "..rs"]
fn deserialise_extensions<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let optional_extensions: Option<Vec<String>> = Option::deserialize(deserializer)?;

    match optional_extensions {
        None => return Ok(None),
        Some(vec) => {
            let cleaned_vec: Vec<String> = vec
                .into_iter()
                .map(|s| s.trim_start_matches('.').to_string())
                .collect();
            Ok(Some(cleaned_vec))
        }
    }
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize, Debug)]
pub struct Config {
    // The user needn't setup any subgroups if they don't wish to
    #[serde(flatten)]
    pub subgroups: Option<PathConfigMap>,
}

#[derive(Deserialize, Debug)]
pub struct PathConfig {
    pub directory: PathBuf,
    // Skip serialising if a PathConfig needs to be generated
    // on the fly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_to_delete: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_to_keep: Option<Vec<String>>,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub delete_hidden: bool,
}

impl PathConfig {
    // Simplified constructor for manual instantiation with just the directory
    pub fn new(directory: PathBuf, use_relative_path: bool) -> Self {
        // Use absolute path by default
        let directory_path = if use_relative_path {
            directory
        } else {
            match canonicalize(&directory) {
                Ok(path) => path,
                Err(err) => panic!("Failed to canonicalize directory: {}", err),
            }
        };

        PathConfig {
            directory: directory_path,
            extensions_to_delete: None, // Default to None
            extensions_to_keep: None,   // Default to None
            recursive: false,           // Default to false
            delete_hidden: false,       // Default to false
        }
    }
}
