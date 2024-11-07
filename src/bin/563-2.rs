use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::sum(&root, &mut ans);
        ans
    }

    fn sum(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let l = Self::sum(&node.left, ans);
            let r = Self::sum(&node.right, ans);
            *ans += (l - r).abs();
            node.val + l + r
        } else {
            0
        }
    }

    // Helper function to build a binary tree from a vector
    pub fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = vec![Rc::clone(&root)];
        let mut i = 1;

        while i < values.len() {
            if let Some(node) = queue.pop() {
                if let Some(val) = values[i] {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left_child));
                    queue.insert(0, left_child);
                }
                i += 1;

                if i < values.len() {
                    if let Some(val) = values[i] {
                        let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().right = Some(Rc::clone(&right_child));
                        queue.insert(0, right_child);
                    }
                    i += 1;
                }
            }
        }

        Some(root)
    }
}

fn main() {
    // Input: root = [4,2,9,3,5,null,7]
    let values = vec![
        Some(4),
        Some(2),
        Some(9),
        Some(3),
        Some(5),
        None,
        Some(7),
    ];

    let root = Solution::build_tree(values);
    let tilt = Solution::find_tilt(root);
    println!("The tilt of the tree is: {}", tilt);
}
