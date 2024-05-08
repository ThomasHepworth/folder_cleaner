mod cleaner_cli;
mod errors;
mod subprompts;
mod validation;

use crate::cleaning::track_files_for_deletion_in_given_config;
use crate::configs::config::PathConfig;
use crate::configs::get_user_config_path;
use crate::configs::unwrap_config_groups::fetch_cli_configs;
use crate::configs::{report_user_config_path, CONFIG_FILE_NAME};
use crate::logging::TextOverviewType;
use clap::Parser;
use cleaner_cli::{CleanArgs, Commands, DirectoryArgs, SizeArgs, CLI};
use errors::CLIError;
use std::path::PathBuf;
use std::process;
use subprompts::prompt_user_decision;
use validation::validate_file_path;

pub fn run_cli() {
    match parse_cli_arguments() {
        Ok((configs, text_overview)) => {
            scan_folders(configs, text_overview);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn parse_cli_arguments() -> Result<(Vec<PathConfig>, TextOverviewType), CLIError> {
    let cli_args = CLI::parse();

    match cli_args.command {
        Commands::ConfigPath => {
            report_user_config_path(CONFIG_FILE_NAME);
            process::exit(0); // Exit the program after reporting the path.
        }
        // Returns a result
        Commands::Clean(args) => handle_cleaner_args(&args),
        Commands::Size(args) => handle_size_args(&args),
    }
}

fn handle_cleaner_args(args: &CleanArgs) -> Result<(Vec<PathConfig>, TextOverviewType), CLIError> {
    let configs = get_and_update_path_config(&args.directory_args)?;
    Ok((configs, TextOverviewType::Deletion))
}

fn handle_size_args(args: &SizeArgs) -> Result<(Vec<PathConfig>, TextOverviewType), CLIError> {
    let configs = get_and_update_path_config(&args.directory_args)?;
    Ok((configs, TextOverviewType::Size))
}

fn get_and_update_path_config(args: &DirectoryArgs) -> Result<Vec<PathConfig>, CLIError> {
    let configs = get_path_config_from_key(&args.path_or_config_key, args.relative_path)?;
    let updated_configs = update_configs_with_cli_args(configs, &args);

    Ok(updated_configs)
}

// TODO: Add strategy pattern here - instant deletion + prompt for deletion
// TODO: Handle dir_tree requested => print_directory_tree
fn scan_folders(configs: Vec<PathConfig>, overview_type: TextOverviewType) {
    for config in configs.iter() {
        match track_files_for_deletion_in_given_config(config, &overview_type) {
            Ok((text_summary, file_folder_queue)) => {
                println!("{}", text_summary);
                match overview_type {
                    TextOverviewType::Deletion => {
                        let user_command = prompt_user_decision(&overview_type);
                        user_command.process_command(file_folder_queue);
                    }
                    _ => {}
                }
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
    cleaner_args: &DirectoryArgs,
) -> Vec<PathConfig> {
    configs
        .into_iter()
        .map(|mut config| {
            config.recursive = cleaner_args.recursive;
            config.delete_hidden = cleaner_args.include_hidden;

            if let Some(size) = &cleaner_args.size {
                config.display_units = size.clone();
            }

            config
        })
        .collect()
}
