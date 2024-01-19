use serde::{self, Deserialize, Deserializer};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

pub type PathConfigMap = HashMap<String, Vec<PathConfig>>;

#[derive(Deserialize, Debug)]
pub enum DisplayUnit {
    Bytes,
    KB, // Kilobytes
    MB, // Megabytes
    GB, // Gigabytes
    TB, // Terabytes
}

impl FromStr for DisplayUnit {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() {
            "BYTES" => Ok(DisplayUnit::Bytes),
            "KB" => Ok(DisplayUnit::KB),
            "MB" => Ok(DisplayUnit::MB),
            "GB" => Ok(DisplayUnit::GB),
            "TB" => Ok(DisplayUnit::TB),
            _ => panic!(
                "Invalid display unit: '{}'. Valid units are: bytes, KB, MB, GB, TB.",
                input
            ),
        }
    }
}

impl DisplayUnit {
    pub fn display_total_size(&self, bytes: u64) -> String {
        match self {
            DisplayUnit::Bytes => format!("{} bytes", bytes),
            DisplayUnit::KB => format!("{:.2} KB", bytes as f64 / 1024.0),
            DisplayUnit::MB => format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0)),
            DisplayUnit::GB => format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0)),
            DisplayUnit::TB => format!(
                "{:.2} TB",
                bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
            ),
        }
    }
}

fn deserialize_display_unit<'de, D>(deserializer: D) -> Result<DisplayUnit, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub size: Option<SizeConfig>,
    // The user needn't setup any subgroups if they don't wish to
    #[serde(flatten)]
    pub subgroups: Option<PathConfigMap>,
}

impl Default for SizeConfig {
    // size is optional, so implement the Default trait to provide a default value
    // if the user doesn't provide one.
    fn default() -> Self {
        SizeConfig {
            display: DisplayUnit::MB, // Default to Megabytes
            ignore_extensions: false,
            walk: false,
            skip_hidden: false,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct SizeConfig {
    // All values are optional. If size is provided,
    // defaults will be automatically set.
    #[serde(deserialize_with = "deserialize_display_unit")]
    pub display: DisplayUnit,
    pub ignore_extensions: bool,
    pub walk: bool,
    pub skip_hidden: bool,
}

#[derive(Deserialize, Debug)]
pub struct PathConfig {
    pub directory: PathBuf,
    pub extensions_to_del: Option<Vec<String>>,
    pub extensions_to_keep: Option<Vec<String>>,
}
