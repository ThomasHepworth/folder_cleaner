use super::folder_tree_helpers::{DirTreeLeaf, DirTreeOptions, TreeKey};
use std::collections::VecDeque;

pub type FileSystemStack = VecDeque<DirTreeLeaf>;
pub type FileSystemStackWithPath = VecDeque<(String, DirTreeLeaf)>;

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

    fn get_pointer_and_prefix(depth: usize, is_last: bool) -> (&'static str, &'static str) {
        (
            Self::get_pointer(depth, is_last),
            Self::get_prefix(depth, is_last),
        )
    }
}

fn get_single_tree_leaf(
    leaf: DirTreeLeaf,
    prefix_stack: &mut Vec<&str>,
    tree_suffix: String,
) -> String {
    // Our prefix stack is a vector of spaces and branches.
    // If our stack is longer than the depth, we want to remove
    // it indicates that we've gone up a directory. See the test at
    // bottom of the page for a visual representation of this.
    prefix_stack.truncate(leaf.depth);
    let (pointer, new_prefix) = DirTreeLimbs::get_pointer_and_prefix(leaf.depth, leaf.is_last);
    let current_line = format!(
        "{}{}{}{}",
        prefix_stack.concat(),
        pointer,
        leaf.key.display_key(),
        tree_suffix
    );

    prefix_stack.push(new_prefix);

    current_line + "\n" // Added newline here for consistency
}

pub fn process_folder_tree_stack(stack: FileSystemStack, print_options: &DirTreeOptions) -> String {
    let mut dir_tree = String::new();
    let mut prefix_stack: Vec<&str> = Vec::new();

    for leaf in stack {
        if print_options.skip_leaf(&leaf.key.as_path()) {
            continue;
        }
        let suffix = print_options.get_tree_suffix_str();
        let leaf = get_single_tree_leaf(leaf, &mut prefix_stack, suffix);
        dir_tree += &leaf;
    }

    dir_tree
}

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
                key: TreeKey::StringKey(key.to_string()),
                depth,
                is_last,
            });
        }

        let options = DirTreeOptions::default();
        let directory_tree = process_folder_tree_stack(stack, &options); // Ensure this function returns a String

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
