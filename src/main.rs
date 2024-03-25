mod cleaning;
mod cli_tools;
mod configs;
mod logging;
mod utils;

use cleaning::track_files_for_deletion::track_files_for_deletion;
use cli_tools::unwrap_config_groups::fetch_cli_configs;
use configs::config::DataSizeUnit;
use configs::get_user_config_path;
use logging::deletion_overview::generate_deletion_overview_text;
use logging::folder_tree_helpers::DirTreeOptions;
use logging::process_directory_tree::process_folder_tree_stack;

fn main() {
    // We can be lazy and simply unwrap this for now
    let config_path = get_user_config_path(".nuke.toml").unwrap();
    println!("{}", config_path.display());
    let configs_result = fetch_cli_configs(&config_path, Some("downloads"));
    let (_, downloads_config) = configs_result.unwrap();
    println!("{:#?}", downloads_config);

    for config in downloads_config.iter() {
        let files_for_deletion = track_files_for_deletion(config);

        match files_for_deletion {
            Ok((deletion_queue, deletion_metadata)) => {
                // println!("Deletion queue: {:#?}", deletion_queue);
                let options = DirTreeOptions::default();
                let deletion_tree = process_folder_tree_stack(deletion_queue, &options);
                println!("{}", deletion_tree);

                let unit = DataSizeUnit::MB;
                let deletion_overview =
                    generate_deletion_overview_text(&config, deletion_metadata, &unit);
                println!("{}", deletion_overview);
            }
            Err(e) => {
                eprintln!("Error tracking files for deletion: {}", e);
            }
        }
    }
}
