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

    fn vec_to_tree_node(vec: Vec<Option<i32>>) -> Node {
        if vec.is_empty() || vec[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
        let mut node_queue = VecDeque::new();
        node_queue.push_back(root.clone());

        let mut i = 1;
        while i < vec.len() {
            if let Some(current_node) = node_queue.pop_front() {
                if i < vec.len() {
                    if let Some(val) = vec[i] {
                        let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                        current_node.borrow_mut().left = Some(left_node.clone());
                        node_queue.push_back(left_node);
                    }
                    i += 1;
                }

                if i < vec.len() {
                    if let Some(val) = vec[i] {
                        let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                        current_node.borrow_mut().right = Some(right_node.clone());
                        node_queue.push_back(right_node);
                    }
                    i += 1;
                }
            }
        }

        Some(root)
    }

    fn tree_node_to_vec(root: Node) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let mut node_queue = VecDeque::new();

        if let Some(root_node) = root {
            node_queue.push_back(root_node);
        }

        while !node_queue.is_empty() {
            if let Some(current_node) = node_queue.pop_front() {
                result.push(Some(current_node.borrow().val));

                let left = current_node.borrow().left.clone();
                let right = current_node.borrow().right.clone();

                if left.is_some() || right.is_some() {
                    node_queue.push_back(left.unwrap_or_else(|| Rc::new(RefCell::new(TreeNode::new(0)))));
                    node_queue.push_back(right.unwrap_or_else(|| Rc::new(RefCell::new(TreeNode::new(0)))));
                }
            } else {
                result.push(None);
            }
        }

        while result.last() == Some(&None) {
            result.pop();
        }

        result
    }
}

fn main() {
    let input_values = vec![5, 4, 9, 1, 10, 0, 7];
    let root: Vec<Option<i32>> = input_values
        .into_iter()
        .map(|x| if x == 0 { None } else { Some(x) })
        .collect();

    let root_node = Solution::vec_to_tree_node(root);
    let modified_root = Solution::replace_value_in_tree(root_node);
    let output = Solution::tree_node_to_vec(modified_root);

    let output: Vec<String> = output
        .into_iter()
        .map(|x| x.map_or_else(|| "null".to_string(), |i| i.to_string()))
        .collect();
    println!("Output: [{}]", output.join(", "));
}
