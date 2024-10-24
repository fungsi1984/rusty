use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Node,
    right: Node,
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
    fn replace_value_in_tree(root: Node) -> Node {
        if root.is_none() {
            return root;
        }

        let mut node_queue = VecDeque::new();
        node_queue.push_back(root.clone().unwrap());

        while !node_queue.is_empty() {
            let level_size = node_queue.len();
            let mut current_level_sum = 0;

            // First, calculate the sum of all nodes at the current level.
            for i in 0..level_size {
                if let Some(node) = node_queue.get(i) {
                    current_level_sum += node.borrow().val;
                }
            }

            // Now process each node, updating its value to the sum of siblings
            for _ in 0..level_size {
                if let Some(current_node) = node_queue.pop_front() {
                    let current_val = current_node.borrow().val;
                    current_node.borrow_mut().val = current_level_sum - current_val;

                    // Add children to the queue
                    let left = current_node.borrow().left.clone();
                    let right = current_node.borrow().right.clone();

                    if let Some(left_node) = left {
                        node_queue.push_back(left_node);
                    }
                    if let Some(right_node) = right {
                        node_queue.push_back(right_node);
                    }
                }
            }
        }

        root
    }
}

fn vec_to_tree(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() || vec[0] == -1 {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(vec[0])));
    let mut nodes = vec![root.clone()];
    let mut i = 1;

    while i < vec.len() {
        if let Some(current_node) = nodes.get(0) {
            let current_node = current_node.clone();
            nodes.remove(0);

            // Left child
            if i < vec.len() && vec[i] != -1 {
                let left_node = Rc::new(RefCell::new(TreeNode::new(vec[i])));
                current_node.borrow_mut().left = Some(left_node.clone());
                nodes.push(left_node);
            }
            i += 1;

            // Right child
            if i < vec.len() && vec[i] != -1 {
                let right_node = Rc::new(RefCell::new(TreeNode::new(vec[i])));
                current_node.borrow_mut().right = Some(right_node.clone());
                nodes.push(right_node);
            }
            i += 1;
        }
    }

    Some(root)
}

fn tree_to_vec(root: Node) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    
    if let Some(node) = root {
        queue.push_back(Some(node));
    }

    while !queue.is_empty() {
        if let Some(current_node) = queue.pop_front() {
            if let Some(node) = current_node {
                let node_ref = node.borrow();
                result.push(Some(node_ref.val));

                if node_ref.left.is_some() || node_ref.right.is_some() {
                    queue.push_back(node_ref.left.clone());
                    queue.push_back(node_ref.right.clone());
                }
            } else {
                result.push(None);
            }
        }
    }

    result
}

fn main() {
    // Example usage:
    let root = vec_to_tree(vec![5, 4, 9, 1, 10, -1, 7]);
    let modified_root = Solution::replace_value_in_tree(root);

    // Convert the modified tree back to a vector and print it
    let result = tree_to_vec(modified_root);
    println!("{:?}", result);
}
