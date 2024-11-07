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
}

fn main() {
    // Creating a binary tree:
    //         1
    //        / \
    //       2   3
    //      / \
    //     4   5

    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    }));

    // Calculate the tilt of the binary tree
    let tilt = Solution::find_tilt(Some(root));
    println!("The tilt of the tree is: {}", tilt);
}
