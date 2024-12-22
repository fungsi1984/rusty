use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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

struct Solution;

impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.clone() {
            let left = root.borrow().left.clone();
            let right = root.borrow().right.clone();
            Self::dfs(left, right, true);
        }
        root
    }

    fn dfs(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>, is_odd_level: bool) {
        if left.is_none() {
            return;
        }

        let left = left.unwrap();
        let right = right.unwrap();

        if is_odd_level {
            let mut left_mut = left.borrow_mut();
            let mut right_mut = right.borrow_mut();
            std::mem::swap(&mut left_mut.val, &mut right_mut.val);
        }

        Self::dfs(left.borrow().left.clone(), right.borrow().right.clone(), !is_odd_level);
        Self::dfs(left.borrow().right.clone(), right.borrow().left.clone(), !is_odd_level);
    }

    pub fn vec_to_tree(values: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }

        let mut root = Rc::new(RefCell::new(TreeNode::new(values[0])));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < values.len() {
            let parent = queue.pop_front().unwrap();

            if i < values.len() {
                let left = Rc::new(RefCell::new(TreeNode::new(values[i])));
                parent.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
                i += 1;
            }

            if i < values.len() {
                let right = Rc::new(RefCell::new(TreeNode::new(values[i])));
                parent.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
                i += 1;
            }
        }

        Some(root)
    }

    pub fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(root) = root {
            queue.push_back(root);
        }

        while let Some(node) = queue.pop_front() {
            result.push(node.borrow().val);
            if let Some(left) = node.borrow().left.clone() {
                queue.push_back(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                queue.push_back(right);
            }
        }

        result
    }
}

fn main() {
    // Example usage
    let values = vec![2, 3, 5, 8, 13, 21, 34];
    let root = Solution::vec_to_tree(values.clone());

    let reversed_root = Solution::reverse_odd_levels(root.clone());
    let output = Solution::tree_to_vec(reversed_root);

    // Print the input and output in the desired format
    println!("Input: root = {:?}", values);
    println!("Output: {:?}", output);
}
