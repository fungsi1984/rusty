use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
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
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut children: HashSet<i32> = HashSet::new();

        for description in descriptions {
            let parent_value = description[0];
            let child_value = description[1];
            let is_left = description[2] == 1;

            let parent = node_map
                .entry(parent_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent_value))))
                .clone();
            let child = node_map
                .entry(child_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child_value))))
                .clone();

            if is_left {
                parent.borrow_mut().left = Some(child.clone());
            } else {
                parent.borrow_mut().right = Some(child.clone());
            }

            children.insert(child_value);
        }

        for (&value, node) in &node_map {
            if !children.contains(&value) {
                return Some(node.clone());
            }
        }

        None
    }

    pub fn level_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        // this annoy unwrap
        queue.push_back(root.unwrap());

        while let Some(node) = queue.pop_front() {
            let node = node.borrow();
            result.push(node.val);

            if let Some(left) = node.left.clone() {
                queue.push_back(left);
            }
            if let Some(right) = node.right.clone() {
                queue.push_back(right);
            }
        }

        result
    }
}

fn main() {
    let descriptions = vec![
        vec![20, 15, 1], // 20 is the parent of 15, and 15 is the left child
        vec![20, 17, 0], // 20 is the parent of 17, and 17 is the right child
        vec![50, 20, 1], // 50 is the parent of 20, and 20 is the left child
        vec![50, 80, 0], // 50 is the parent of 80, and 80 is the right child
        vec![80, 19, 1], // 80 is the parent of 19, and 19 is the left child
    ];

    let tree = Solution::create_binary_tree(descriptions);
    let output = Solution::level_order_traversal(tree);
    println!("{:?}", output);
}
