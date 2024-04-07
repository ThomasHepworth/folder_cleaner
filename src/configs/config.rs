use serde::{self, Deserialize, Deserializer};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::str::FromStr;

pub type PathConfigMap = HashMap<String, Vec<PathConfig>>;

#[derive(Deserialize, Debug, Clone)]
pub enum DataSizeUnit {
    Bytes,
    KB, // Kilobytes
    MB, // Megabytes
    GB, // Gigabytes
    TB, // Terabytes
}

impl FromStr for DataSizeUnit {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() {
            "BYTES" => Ok(DataSizeUnit::Bytes),
            "KB" => Ok(DataSizeUnit::KB),
            "MB" => Ok(DataSizeUnit::MB),
            "GB" => Ok(DataSizeUnit::GB),
            "TB" => Ok(DataSizeUnit::TB),
            _ => panic!(
                "Invalid display unit: '{}'. Valid units are: bytes, KB, MB, GB, TB.",
                input
            ),
        }
    }
}

impl DataSizeUnit {
    pub fn display_total_size(&self, bytes: u64) -> String {
        match self {
            DataSizeUnit::Bytes => format!("{} bytes", bytes),
            DataSizeUnit::KB => format!("{:.2} KB", bytes as f64 / 1024.0),
            DataSizeUnit::MB => format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0)),
            DataSizeUnit::GB => format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0)),
            DataSizeUnit::TB => format!(
                "{:.2} TB",
                bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
            ),
        }
    }
}

fn deserialise_data_size_unit<'de, D>(deserializer: D) -> Result<DataSizeUnit, D::Error>
// TODO: Migrate to deserialise
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

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

fn default_display_unit() -> DataSizeUnit {
    DataSizeUnit::MB
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
    // All values are optional. If size is provided,
    // defaults will be automatically set.
    #[serde(
        default = "default_display_unit",
        deserialize_with = "deserialise_data_size_unit"
    )]
    pub display_units: DataSizeUnit,
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
            extensions_to_delete: None,      // Default to None
            extensions_to_keep: None,        // Default to None
            recursive: false,                // Default to false
            delete_hidden: false,            // Default to false
            display_units: DataSizeUnit::MB, // Use the default_display function to get the default
        }
    }
}
