mod cli_tools;
mod configs;
mod file_system_caching;
mod size_checker;

use configs::config::{Config, PathConfig};
use std::path::PathBuf;
// use clap::{App, Arg};
// use cli_tools::process_clean_command;

use cli_tools::{get_subgroup, unwrap_all_subgroups};
use configs::extract_user_config;
use size_checker::{calculate_and_report_user_group_file_sizes, calculate_total_size};

use std::collections::HashMap;

use file_system_caching::folder_meta::{crawl_folders_for_metadata, Folder};

fn main() {
    // let (size, subgroups) = extract_user_config(".nuke.toml").unwrap();
    // println!("{:#?}", size);
    // println!("{:#?}", subgroups);
    // calculate_and_report_user_group_file_sizes(&subgroups.unwrap(), &size);

    // use walkdir::WalkDir;
    let path = PathBuf::from("/Users/thomashepworth/Downloads");
    let mut folder = Folder::new(path);
    // println!("{:#?}", &folder);
    // folder.update_folder_metadata();
    // println!("{:#?}", folder.calculate_size());
    // folder.scan_folder();

    let mut folder_map = HashMap::new();
    let root_path = PathBuf::from("/Users/thomashepworth/Downloads");
    // let test_map = crawl_folders_for_metadata(&mut folder_map, &root_path, false);
    let test_map = crawl_folders_for_metadata(&mut folder_map, &root_path, true);
    println!("{:#?}", folder_map);

    // Errors...
    // let root_path = PathBuf::from("/Users/thomashepworth/Downloadings");
    // let test_map = crawl_folders_for_metadata(&mut folder_map, &root_path);
}
