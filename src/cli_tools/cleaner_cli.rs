use clap::Parser;

/// Cleans up folders based on a given path or configuration file.
#[derive(Parser)]
#[command(
    name = "folder_cleaner",
    about = "A safer file cleaner. Generate detailed insights \
        about your folder(s), so you can avoid accidentally deleting data.",
    version = "1.0"
)]
pub struct CleanerCLI {
    /// The path to clean or a configuration key to use.
    #[arg(required_unless_present_any = &["config"], conflicts_with = "config")]
    pub path_or_config_key: Option<String>,

    /// Display the path to your config file
    #[arg(short, long)]
    pub config: bool,

    /// Specifies whether to use the path as a configuration key and skip checking your configuration file 🗝️
    #[arg(short, long)]
    pub use_path: bool,

    /// Print out the file system tree 🌲
    #[arg(short, long)]
    pub tree: bool,

    /// Recursively scan all items within all subfolders. Defaults to false 📁
    #[arg(short)]
    pub recursive: bool,

    /// Whether to delete hidden files and folders. Defaults to false 🕵️
    #[arg(short)]
    pub delete_hidden: bool,

    /// Automatically approve the deletion request ✅
    #[arg(short)]
    pub yes: bool,

    /// Print the size of each file and folder within the deletion tree (requires --tree) 📦
    #[arg(short, long, requires = "tree")]
    pub size: bool,
}
