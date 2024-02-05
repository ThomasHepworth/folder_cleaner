use std::collections::VecDeque;

// TODO: Come up with stack creation strategies

pub struct FileSystemLeaf {
    pub key: String,
    pub depth: usize,
    pub is_last: bool,
}

pub type FileSystemStack = VecDeque<FileSystemLeaf>;

struct FolderTreeLimbs {
    space: &'static str,
    branch: &'static str,
    tee: &'static str,
    last: &'static str,
}

impl FolderTreeLimbs {
    fn new() -> FolderTreeLimbs {
        FolderTreeLimbs {
            space: "    ",
            branch: "│   ",
            tee: "├── ",
            last: "└── ",
        }
    }

    fn get_pointer(&self, depth: usize, is_last: bool) -> &'static str {
        match (depth, is_last) {
            (0, _) => "",
            (_, true) => self.last,
            (_, false) => self.tee,
        }
    }

    fn get_prefix(&self, depth: usize, is_last: bool) -> &'static str {
        match (depth, is_last) {
            (0, _) => "",
            (_, true) => self.space,
            (_, false) => self.branch,
        }
    }
}

pub fn process_folder_tree_stack(mut stack: FileSystemStack) -> String {
    let tree_limbs = FolderTreeLimbs::new();
    let mut final_tree = String::new();
    let mut prefix_stack: Vec<&str> = Vec::new();

    while let Some(FileSystemLeaf {
        key,
        depth,
        is_last,
    }) = stack.pop_front()
    {
        while prefix_stack.len() > depth {
            prefix_stack.pop();
        }

        let pointer = tree_limbs.get_pointer(depth, is_last);

        // Construct the current line
        let current_line = format!("{}{}{}", prefix_stack.concat(), pointer, key);
        final_tree.push_str(&current_line);
        final_tree.push('\n');

        // Update the prefix stack
        let new_prefix = tree_limbs.get_prefix(depth, is_last);
        prefix_stack.push(new_prefix);
    }

    final_tree
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
            stack.push_back(FileSystemLeaf {
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
