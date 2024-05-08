use inquire::{InquireError, Select};
use std::fmt;
use std::process;

use crate::cleaning::delete_files_scheduled_for_deletion;
use crate::logging::process_directory_tree::FileSystemStack;
use crate::logging::{print_directory_tree, TextOverviewType};

pub enum PromptArg {
    Delete,
    Exit,
    Tree,
}

impl fmt::Display for PromptArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PromptArg {
    fn as_str(&self) -> &str {
        match self {
            PromptArg::Delete => "Delete 🗑️",
            PromptArg::Exit => "Exit 🚪",
            PromptArg::Tree => "Print directory tree 🌲",
        }
    }

    pub fn process_command(&self, directory_stack: FileSystemStack) {
        match self {
            PromptArg::Delete => {
                match delete_files_scheduled_for_deletion(directory_stack) {
                    Ok(_) => {
                        println!("All files were successfully deleted.");
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
                process::exit(0);
            }
            PromptArg::Exit => {
                println!("Exiting the program");
                process::exit(0);
            }
            PromptArg::Tree => {
                print_directory_tree(directory_stack);
            }
        }
    }
}

pub fn prompt_user_decision(text_overview_type: &TextOverviewType) -> PromptArg {
    let deletion_args: Vec<PromptArg> = vec![PromptArg::Delete, PromptArg::Exit, PromptArg::Tree];
    let size_args: Vec<PromptArg> = vec![PromptArg::Exit, PromptArg::Tree];

    let deletion_prompt = "👉 Would you like to proceed with the deletion?";
    let size_prompt = "👉 Would you like to see the directory tree representation?";

    let (options, prompt) = match text_overview_type {
        TextOverviewType::Deletion => (deletion_args, deletion_prompt),
        TextOverviewType::Size => (size_args, size_prompt),
    };

    let argument: Result<PromptArg, InquireError> = Select::new(prompt, options).prompt();

    match argument {
        // Return the option selected by the user
        Ok(choice) => choice,
        Err(e) => {
            eprintln!("Error encountered during selection: {}", e);
            process::exit(1);
        }
    }
}
