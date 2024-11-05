use std::collections::{HashMap, VecDeque};

// Definition for a binary tree node
#[derive(Debug, Default, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {
    // Array to store the maximum height of the tree after removing each node
    max_height_after_removal: HashMap<i32, i32>,
    current_max_height: i32,
}

impl Solution {
    fn new() -> Self {
        Solution {
            max_height_after_removal: HashMap::new(),
            current_max_height: 0,
        }
    }

    fn tree_queries(&mut self, root: Option<Box<TreeNode>>, queries: Vec<i32>) -> Vec<i32> {
        // Traverse from left to right
        self.traverse_left_to_right(&root, 0);
        // Reset for the second traversal
        self.current_max_height = 0;
        // Traverse from right to left
        self.traverse_right_to_left(&root, 0);

        // Process queries and build the result vector
        queries
            .into_iter()
            .map(|query| *self.max_height_after_removal.get(&query).unwrap_or(&0))
            .collect()
    }

    // Left to right traversal
    fn traverse_left_to_right(&mut self, node: &Option<Box<TreeNode>>, current_height: i32) {
        if let Some(node) = node {
            // Store the maximum height if this node were removed
            self.max_height_after_removal
                .insert(node.val, self.current_max_height);

            // Update the current maximum height
            self.current_max_height = self.current_max_height.max(current_height);

            // Traverse left subtree first, then right
            self.traverse_left_to_right(&node.left, current_height + 1);
            self.traverse_left_to_right(&node.right, current_height + 1);
        }
    }

    // Right to left traversal
    fn traverse_right_to_left(&mut self, node: &Option<Box<TreeNode>>, current_height: i32) {
        if let Some(node) = node {
            // Update the maximum height if this node were removed
            let entry = self
                .max_height_after_removal
                .entry(node.val)
                .or_insert(0);
            *entry = (*entry).max(self.current_max_height);

            // Update the current maximum height
            self.current_max_height = self.current_max_height.max(current_height);

            // Traverse right subtree first, then left
            self.traverse_right_to_left(&node.right, current_height + 1);
            self.traverse_right_to_left(&node.left, current_height + 1);
        }
    }
}

// Helper function to build tree from level-order representation
fn vec_to_tree_node(data: Vec<i32>) -> Option<Box<TreeNode>> {
    if data.is_empty() || data[0] == -1 {
        return None;
    }

    let root = Box::new(TreeNode::new(data[0]));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < data.len() {
        if let Some(mut current) = queue.pop_front() {
            // Left child
            if data[i] != -1 {
                let left_child = Box::new(TreeNode::new(data[i]));
                current.left = Some(left_child.clone());
                queue.push_back(left_child);
            }
            i += 1;

            // Right child
            if i < data.len() && data[i] != -1 {
                let right_child = Box::new(TreeNode::new(data[i]));
                current.right = Some(right_child.clone());
                queue.push_back(right_child);
            }
            i += 1;
        }
    }

    Some(root)
}

fn main() {
    let root_values = vec![5, 8, 9, 2, 1, 3, 7, 4, 6];
    let root_node = vec_to_tree_node(root_values);

    // Queries
    let queries = vec![3, 2, 4, 8];

    // Execute tree_queries and print results
    let mut solution = Solution::new();
    let result = solution.tree_queries(root_node, queries);

    println!("Output: {:?}", result); // Expected output: [3, 2, 3, 2]
}
