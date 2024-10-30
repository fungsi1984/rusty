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
        if let Some(root_node) = root.clone() {
            node_queue.push_back(root_node);
        }

        let mut previous_level_sum = root.as_ref().unwrap().borrow().val;

        while !node_queue.is_empty() {
            let level_size = node_queue.len();
            let mut current_level_sum = 0;

            for _ in 0..level_size {
                if let Some(current_node) = node_queue.pop_front() {
                    let current_val = current_node.borrow().val;
                    current_node.borrow_mut().val = previous_level_sum - current_val;

                    let left = current_node.borrow().left.clone();
                    let right = current_node.borrow().right.clone();

                    let sibling_sum =
                        left.as_ref().map_or(0, |l| l.borrow().val) + right.as_ref().map_or(0, |r| r.borrow().val);

                    if let Some(left_node) = left {
                        current_level_sum += left_node.borrow().val;
                        left_node.borrow_mut().val = sibling_sum;
                        node_queue.push_back(left_node);
                    }
                    if let Some(right_node) = right {
                        current_level_sum += right_node.borrow().val;
                        right_node.borrow_mut().val = sibling_sum;
                        node_queue.push_back(right_node);
                    }
                }
            }

            previous_level_sum = current_level_sum;
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

fn main() {
    // Example usage:
    let root = vec_to_tree(vec![5, 4, 9, 1, 10, -1, 7]);
    let modified_root = Solution::replace_value_in_tree(root);

    // You can add code here to print or validate the modified tree
}
