use super::folder_tree_helpers::{
    DirTreeLeaf, DirTreeOptions, FileSystemStack, FileSystemStackWithPath, TreeSuffix,
};
use crate::file_system_caching::folder_meta::{Folder, SortKey};
use crate::utils::join_path_conditionally;
use std::collections::{HashMap, VecDeque}; // Add missing import for the `utils` module

struct DirTreeLimbs;

impl DirTreeLimbs {
    // Define these as constants within the impl block if they're only used here
    const SPACE: &'static str = "    ";
    const BRANCH: &'static str = "│   ";
    const TEE: &'static str = "├── ";
    const LAST: &'static str = "└── ";

    // Now an associated function (static method)
    fn get_pointer(depth: usize, is_last: bool) -> &'static str {
        match (depth, is_last) {
            (0, _) => ".",
            (_, true) => Self::LAST,
            (_, false) => Self::TEE,
        }
    }

    // Now an associated function (static method)
    fn get_prefix(depth: usize, is_last: bool) -> &'static str {
        match (depth, is_last) {
            (0, _) => "",
            (_, true) => Self::SPACE,
            (_, false) => Self::BRANCH,
        }
    }
}

fn get_single_tree_leaf(
    leaf: DirTreeLeaf,
    prefix_stack: &mut Vec<&str>,
    tree_suffix: Option<TreeSuffix>,
) -> String {
    // Our prefix stack is a vector of spaces and branches.
    // If our stack is longer than the depth, we want to remove
    // it indicates that we've gone up a directory. See the test at
    // bottom of the page for a visual representation of this.
    prefix_stack.truncate(leaf.depth);
    let pointer = DirTreeLimbs::get_pointer(leaf.depth, leaf.is_last);
    let suffix = tree_suffix.map_or_else(String::new, TreeSuffix::get_suffix_str);
    let current_line = format!("{}{}{}{}", prefix_stack.concat(), pointer, leaf.key, suffix);

    let new_prefix = DirTreeLimbs::get_prefix(leaf.depth, leaf.is_last);
    prefix_stack.push(new_prefix);

    current_line + "\n" // Added newline here for consistency
}

fn update_directory_tree(mut dir_tree: String, current_line: String) -> String {
    dir_tree += &current_line; // Append the current line to the directory tree string
    dir_tree // Return the updated directory tree
}

fn process_folder_tree_stack(stack: FileSystemStack) -> String {
    let mut dir_tree = String::new();
    let mut prefix_stack: Vec<&str> = Vec::new();

    for leaf in stack {
        let leaf = get_single_tree_leaf(leaf, &mut prefix_stack, None);
        dir_tree = update_directory_tree(dir_tree, leaf);
    }

    dir_tree
}

pub fn create_dir_tree_from_folder_metadata_hashmap(
    root_folder_path: String,
    folder_hash_map: &HashMap<String, Folder>,
    options: &DirTreeOptions,
) -> String {
    // Uses DFS to crawl our tree
    let mut directory_tree = String::new();
    let mut prefix_stack: Vec<&str> = Vec::new();
    let mut dir_tree_stack: FileSystemStackWithPath = VecDeque::new();

    let root_leaf = DirTreeLeaf::new_root(root_folder_path);

    // TODO: Remove this string for files -> Make this an optional
    dir_tree_stack.push_front(("".to_owned(), root_leaf));

    while let Some((path, current_leaf)) = dir_tree_stack.pop_front() {
        // Avoids extra "/" and invalid paths on other systems -> "\" vs "/"
        let map_path = join_path_conditionally(&path, &current_leaf.key);

        if let Some(folder) = folder_hash_map.get(&map_path) {
            add_subfolders_to_directory_tree_stack(
                map_path,
                folder,
                &current_leaf,
                &mut dir_tree_stack,
            );
            // if options.display_files {
            //     add_files_to_directory_tree_stack(folder, &current_leaf, &mut dir_tree_stack);
            // }
            let tree_suffix = options.generate_tree_suffix(&folder.metadata);
            let leaf = get_single_tree_leaf(current_leaf, &mut prefix_stack, tree_suffix);
            directory_tree = update_directory_tree(directory_tree, leaf);
        }
    }

    directory_tree
}

fn add_subfolders_to_directory_tree_stack(
    folder_path: String,
    folder_data: &Folder,
    current_leaf: &DirTreeLeaf,
    dir_tree_stack: &mut FileSystemStackWithPath,
) {
    folder_data
        .metadata
        .subfolders
        .iter()
        .rev()
        .enumerate()
        .for_each(|(index, subfolder_path)| {
            let leaf = DirTreeLeaf {
                key: subfolder_path.clone(),
                depth: current_leaf.depth + 1,
                is_last: index == 0,
            };
            dir_tree_stack.push_front((folder_path.clone(), leaf));
        });
}

// fn add_files_to_directory_tree_stack(
//     folder_data: Folder,
//     current_leaf: &DirTreeLeaf,
//     dir_tree_stack: &mut FileSystemStackWithPath,
// ) {
//     // For now, sort by size
//     folder_data.metadata.sort_files(SortKey::Size);

//     folder_data.metadata.files.iter().for_each(|file| {
//         let leaf = DirTreeLeaf {
//             key: file.file_name.clone(),
//             depth: current_leaf.depth + 1,
//             is_last: true,
//         };
//         dir_tree_stack.push_front(("".to_owned(), leaf));
//     });
// }

#[cfg(test)]
mod tests {
    use super::*; // Adjust based on your module structure
    use std::collections::VecDeque;

    #[test]
    fn test_process_folder_tree_stack() {
        let folder_leaves = vec![
            ("main_folder", 0, true),
            ("file01.txt", 1, false),
            ("file02.txt", 1, false),
            ("folder_sub1", 1, false),
            ("file03.txt", 2, false),
            ("file04.txt", 2, false),
            ("file05.txt", 2, false),
            ("folder_sub1-1", 2, false),
            ("file09.txt", 3, false),
            ("file10.txt", 3, false),
            ("file11.txt", 3, true),
            ("testing", 2, true),
            ("folder_sub2", 1, true),
            ("file06.txt", 2, false),
            ("file07.txt", 2, false),
            ("file08.txt", 2, false),
            ("folder_sub2-1", 2, true),
        ];

        let mut stack: FileSystemStack = VecDeque::new();
        for (key, depth, is_last) in folder_leaves {
            stack.push_back(DirTreeLeaf {
                key: key.to_owned(),
                depth,
                is_last,
            });
        }

        let directory_tree = process_folder_tree_stack(stack); // Ensure this function returns a String

        let expected_output = "\
main_folder
├── file01.txt
├── file02.txt
├── folder_sub1
│   ├── file03.txt
│   ├── file04.txt
│   ├── file05.txt
│   ├── folder_sub1-1
│   │   ├── file09.txt
│   │   ├── file10.txt
│   │   └── file11.txt
│   └── testing
└── folder_sub2
    ├── file06.txt
    ├── file07.txt
    ├── file08.txt
    └── folder_sub2-1
";

        assert_eq!(directory_tree, expected_output);
    }
}
