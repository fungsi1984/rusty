// tree_refactor.rs

/// A module for working with trees, including serialization and deserialization.
///
/// Provides functionality for converting between vector representations and tree structures,
/// as well as modifying tree node values based on sibling sums.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Represents a node in the tree with a value and optional left and right children.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Node,
    right: Node,
}

impl TreeNode {
    /// Creates a new `TreeNode` with the given value and no children.
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Type alias for an optional reference-counted `TreeNode`.
type Node = Option<Rc<RefCell<TreeNode>>>;

/// A solution struct containing methods for tree manipulation.
struct Solution;

impl Solution {
    /// Replaces each node's value in the tree with the sum of its siblings' values at each level.
    ///
    /// # Returns
    /// The modified tree.
    fn replace_value_in_tree(root: Node) -> Node {
        if root.is_none() {
            return root;
        }
        
        let mut node_queue = VecDeque::new();
        node_queue.push_back(Rc::clone(&root.as_ref().unwrap()));
        
        // Set the root's value to 0 as per the problem's expectation
        root.as_ref().unwrap().borrow_mut().val = 0;

        while!node_queue.is_empty() {
            let level_size = node_queue.len();
            let mut current_level_sum = 0;
            let mut nodes_at_level = vec![];

            // First, calculate the sum of all nodes at the current level.
            for _ in 0..level_size {
                if let Some(node) = node_queue.pop_front() {
                    current_level_sum += node.borrow().val;
                    nodes_at_level.push(Rc::clone(&node));
                    
                    // Add children to the queue
                    if let Some(left_node) = node.borrow().left.clone() {
                        node_queue.push_back(left_node);
                    }
                    if let Some(right_node) = node.borrow().right.clone() {
                        node_queue.push_back(right_node);
                    }
                }
            }

            // Now update the value of each node to be the sum of its siblings
            for node in nodes_at_level {
                let current_val = node.borrow().val;
                node.borrow_mut().val = current_level_sum - current_val;
            }
        }

        root
    }
}

/// Converts a vector representation into a tree.
///
/// The vector is expected to be a breadth-first traversal of the tree,
/// where `null` represents a null node.
const NULL_NODE: &str = "null";

fn vec_to_tree(vec: Vec<&str>) -> Node {
    if vec.is_empty() || vec[0] == NULL_NODE {
        return None;
    }
    
    let root = Rc::new(RefCell::new(TreeNode::new(vec[0].parse().unwrap())));
    let mut nodes = vec![Rc::clone(&root)];
    let mut i = 1;

    while i < vec.len() {
        if let Some(current_node) = nodes.get(0) {
            let current_node = Rc::clone(current_node);
            nodes.remove(0);

            // Left child
            if i < vec.len() && vec[i]!= NULL_NODE {
                let left_node = Rc::new(RefCell::new(TreeNode::new(vec[i].parse().unwrap())));
                current_node.borrow_mut().left = Some(Rc::clone(&left_node));
                nodes.push(left_node);
            }
            i += 1;

            // Right child
            if i < vec.len() && vec[i]!= NULL_NODE {
                let right_node = Rc::new(RefCell::new(TreeNode::new(vec[i].parse().unwrap())));
                current_node.borrow_mut().right = Some(Rc::clone(&right_node));
                nodes.push(right_node);
            }
            i += 1;
        }
    }

    Some(root)
}

/// Converts a tree into a vector representation, using `null` for null nodes.
fn tree_to_vec(root: Node) -> Vec<String> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    
    if let Some(node) = root {
        queue.push_back(Some(node));
    }

    while let Some(current_node) = queue.pop_front() {
        match current_node {
            Some(node) => {
                let node_ref = node.borrow();
                result.push(node_ref.val.to_string());
                
                queue.push_back(node_ref.left.clone());
                queue.push_back(node_ref.right.clone());
            },
            None => result.push(NULL_NODE.to_string()),
        }
    }
    
    result
}

fn main() {
    // Example usage:
    let original_tree = vec!["5", "4", "9", "1", "10", "null", "7"];
    let root = vec_to_tree(original_tree.clone());
    let modified_root = Solution::replace_value_in_tree(root);
    
    // Convert the modified tree back to a vector and print it
    let result = tree_to_vec(modified_root);
    
    println!("Input: root = {:?}", original_tree);
    println!("Output: {:?}", result);
}