use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn tree_queries(
        root: Option<Rc<RefCell<TreeNode>>>,
        queries: Vec<i32>,
    ) -> Vec<i32> {
        let mut max_height_after_removal = HashMap::new();
        let mut current_max_height = 0;

        // First traversal (left to right)
        Solution::traverse_left_to_right(&root, 0, &mut current_max_height, &mut max_height_after_removal);
        
        // Reset for the second traversal
        current_max_height = 0;

        // Second traversal (right to left)
        Solution::traverse_right_to_left(&root, 0, &mut current_max_height, &mut max_height_after_removal);

        // Process queries
        queries
            .into_iter()
            .map(|query| *max_height_after_removal.get(&query).unwrap_or(&0))
            .collect()
    }

    fn traverse_left_to_right(
        node: &Option<Rc<RefCell<TreeNode>>>,
        current_height: i32,
        current_max_height: &mut i32,
        max_height_after_removal: &mut HashMap<i32, i32>,
    ) {
        if let Some(n) = node {
            let n = n.borrow();
            // Store the maximum height if this node were removed
            max_height_after_removal.insert(n.val, *current_max_height);

            // Update the current maximum height
            *current_max_height = (*current_max_height).max(current_height);

            // Traverse left subtree first, then right
            Solution::traverse_left_to_right(&n.left, current_height + 1, current_max_height, max_height_after_removal);
            Solution::traverse_left_to_right(&n.right, current_height + 1, current_max_height, max_height_after_removal);
        }
    }

    fn traverse_right_to_left(
        node: &Option<Rc<RefCell<TreeNode>>>,
        current_height: i32,
        current_max_height: &mut i32,
        max_height_after_removal: &mut HashMap<i32, i32>,
    ) {
        if let Some(n) = node {
            let n = n.borrow();
            // Update the maximum height if this node were removed
            let entry = max_height_after_removal.entry(n.val).or_insert(0);
            *entry = (*entry).max(*current_max_height);

            // Update the current maximum height
            *current_max_height = (*current_max_height).max(current_height);

            // Traverse right subtree first, then left
            Solution::traverse_right_to_left(&n.right, current_height + 1, current_max_height, max_height_after_removal);
            Solution::traverse_right_to_left(&n.left, current_height + 1, current_max_height, max_height_after_removal);
        }
    }
}

// Helper function to build a tree from level-order representation
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
