use directories::ProjectDirs;
use std::path::PathBuf;

enum FolderType {
    Config,
    Data,
}

fn get_project_folder(folder_type: FolderType) -> Option<PathBuf> {
    let crate_name = env!("CARGO_PKG_NAME");

    ProjectDirs::from("dev", "tom-hepworth", crate_name).map(|proj_dirs| {
        match folder_type {
            FolderType::Config => proj_dirs.config_dir(),
            FolderType::Data => proj_dirs.data_dir(),
        }
        .to_path_buf()
    })
}
