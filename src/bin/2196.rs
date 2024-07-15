use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
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
}

fn main() {
    let descriptions = vec![
        vec![1, 2, 1], // 1 is the parent of 2, and 2 is the left child
        vec![1, 3, 0], // 1 is the parent of 3, and 3 is the right child
        vec![2, 4, 1], // 2 is the parent of 4, and 4 is the left child
    ];

    let tree = Solution::create_binary_tree(descriptions);
    println!("{:?}", tree);
}
