mod cleaner_cli;
mod errors;
mod validation;

use crate::cleaning::{print_deletion_tree, process_config};
use crate::configs::config::PathConfig;
use crate::configs::get_user_config_path;
use crate::configs::unwrap_config_groups::fetch_cli_configs;
use crate::configs::{report_user_config_path, CONFIG_FILE_NAME};
use clap::Parser;
use cleaner_cli::{CleanArgs, Commands, CLI};
use errors::CLIError;
use std::path::PathBuf;
use std::process;
use validation::validate_file_path;

fn parse_cleaner_cli() -> Result<Vec<PathConfig>, CLIError> {
    let cli_args = CLI::parse();

    match cli_args.command {
        Commands::ConfigPath => {
            report_user_config_path(CONFIG_FILE_NAME);
            process::exit(0); // Exit the program after reporting the path.
        }
        Commands::Clean(args) => {
            let configs = get_path_config_from_key(&args.path_or_config_key, args.relative_path)?;
            let updated_configs = update_configs_with_cli_args(configs, &args);
            Ok(updated_configs)
        }
    }
}

pub fn run_clean_up() {
    match parse_cleaner_cli() {
        Ok(configs) => {
            clean_folders(configs);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn clean_folders(configs: Vec<PathConfig>) {
    for config in configs.iter() {
        match process_config(config) {
            Ok((deletion_overview, deletion_queue)) => {
                println!("{}", deletion_overview);
                print_deletion_tree(deletion_queue);
            }
            Err(e) => {
                eprintln!("Failed to process the configuration for deletion: {}", e);
                process::exit(1);
            }
        }
    }
}

fn get_path_config_from_key(
    user_key: &str,
    relative_path: bool,
) -> Result<Vec<PathConfig>, CLIError> {
    let config_path = get_user_config_path(CONFIG_FILE_NAME)?;

    // TODO: Better messaging here?
    // Better error handling w/ unwrap
    match fetch_cli_configs(&config_path, Some(user_key)) {
        Ok(config_group) => Ok(config_group),
        Err(_) if validate_file_path(user_key) => Ok(vec![PathConfig::new(
            PathBuf::from(user_key),
            // Dictates how the path is displayed to the user
            relative_path,
        )]),
        Err(_) => Err(CLIError::PathOrConfigError(user_key.to_string())),
    }
}

fn update_configs_with_cli_args(
    configs: Vec<PathConfig>,
    cleaner_args: &CleanArgs,
) -> Vec<PathConfig> {
    configs
        .into_iter()
        .map(|mut config| {
            config.recursive = cleaner_args.recursive;
            config.delete_hidden = cleaner_args.delete_hidden;

            if let Some(size) = &cleaner_args.size {
                config.display_units = size.clone();
            }

            config
        })
        .collect()
}
