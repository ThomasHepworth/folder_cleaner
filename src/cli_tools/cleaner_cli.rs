use crate::configs::config::DataSizeUnit;
use clap::{Parser, Subcommand};

/// Cleans up folders based on a given path or configuration file.
#[derive(Parser)]
#[command(
    name = "folder_cleaner",
    about = "A safer file cleaner. Generate detailed insights \
        about your folder(s), so you can avoid accidentally deleting data.",
    version = "1.0"
)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct DirectoryArgs {
    /// The path or configuration key to use.
    #[arg(required = true)]
    pub path_or_config_key: String,

    /// Print out the file system tree 🌲
    #[arg(short, long)]
    pub tree: bool,

    /// Recursively scan all items within all subfolders 📁
    #[arg(short, default_value_t = false)]
    pub recursive: bool,

    /// Whether to include hidden files and folders.
    #[arg(short, default_value_t = false)]
    pub include_hidden: bool,

    /// Determines the path display format. If true, the path will be relative to the current directory.
    #[arg(long, aliases = ["relative", "relativepath"])]
    pub relative_path: bool,

    #[arg(short, long, requires = "tree", verbatim_doc_comment, value_parser = clap::value_parser!(DataSizeUnit))]
    /// Print the size of each file and folder within the deletion tree (requires --tree) 📦
    /// Accepts a value to specify the size unit: Bytes, KB, MB, GB or TB.
    pub size: Option<DataSizeUnit>,
}

#[derive(Parser)]
pub struct CleanArgs {
    #[clap(flatten)]
    pub directory_args: DirectoryArgs,

    /// Automatically approve the deletion request.
    #[arg(short)]
    pub yes: bool,
}

#[derive(Parser)]
pub struct SizeArgs {
    #[clap(flatten)]
    pub directory_args: DirectoryArgs,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clean a directory based on a path or configuration key.
    Clean(CleanArgs),

    /// Show the size of a directory based on a path or configuration key.
    Size(SizeArgs),

    /// Display the path to your configuration file.
    ConfigPath,
}
