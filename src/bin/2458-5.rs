use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {
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

    pub fn tree_queries(
        root: Option<Rc<RefCell<TreeNode>>>,
        queries: Vec<i32>,
    ) -> Vec<i32> {
        let mut solution = Solution::new();
        // Traverse from left to right
        solution.traverse_left_to_right(root.clone(), 0);
        // Reset for the second traversal
        solution.current_max_height = 0;
        // Traverse from right to left
        solution.traverse_right_to_left(root, 0);

        // Process queries and build the result vector
        queries
            .into_iter()
            .map(|query| *solution.max_height_after_removal.get(&query).unwrap_or(&0))
            .collect()
    }

    fn traverse_left_to_right(&mut self, node: Option<Rc<RefCell<TreeNode>>>, current_height: i32) {
        if let Some(n) = node {
            let n = n.borrow();
            // Store the maximum height if this node were removed
            self.max_height_after_removal
                .insert(n.val, self.current_max_height);

            // Update the current maximum height
            self.current_max_height = self.current_max_height.max(current_height);

            // Traverse left subtree first, then right
            self.traverse_left_to_right(n.left.clone(), current_height + 1);
            self.traverse_left_to_right(n.right.clone(), current_height + 1);
        }
    }

    fn traverse_right_to_left(&mut self, node: Option<Rc<RefCell<TreeNode>>>, current_height: i32) {
        if let Some(n) = node {
            let n = n.borrow();
            // Update the maximum height if this node were removed
            let entry = self
                .max_height_after_removal
                .entry(n.val)
                .or_insert(0);
            *entry = (*entry).max(self.current_max_height);

            // Update the current maximum height
            self.current_max_height = self.current_max_height.max(current_height);

            // Traverse right subtree first, then left
            self.traverse_right_to_left(n.right.clone(), current_height + 1);
            self.traverse_right_to_left(n.left.clone(), current_height + 1);
        }
    }
}

// Helper function to build tree from level-order representation
fn vec_to_tree_node(data: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() || data[0] == -1 {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(data[0])));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < data.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if data[i] != -1 {
                let left_child = Rc::new(RefCell::new(TreeNode::new(data[i])));
                current.borrow_mut().left = Some(left_child.clone());
                queue.push_back(left_child);
            }
            i += 1;

            // Right child
            if i < data.len() && data[i] != -1 {
                let right_child = Rc::new(RefCell::new(TreeNode::new(data[i])));
                current.borrow_mut().right = Some(right_child.clone());
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
    let result = Solution::tree_queries(root_node, queries);

    println!("Output: {:?}", result); // Expected output: [3, 2, 3, 2]
}
