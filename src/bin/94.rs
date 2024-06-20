use std::cell::RefCell;
use std::rc::Rc;

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
      right: None
    }
  }
}

type Node = Option<Rc<RefCell<TreeNode>>>;
struct Solution;

impl Solution {
    // Function to perform inorder traversal of the binary tree
    pub fn inorder_traversal(root: Node) -> Vec<i32> {
        let mut res = vec![];

        // Helper function to perform recursive traversal
        fn traversal(node: Node, res: &mut Vec<i32>) {
            if let Some(n) = node {
                traversal(n.borrow().left.clone(), res); // Traverse left subtree
                res.push(n.borrow().val); // Push current node's value
                traversal(n.borrow().right.clone(), res); // Traverse right subtree
            }
        }

        // Start traversal from the root
        traversal(root, &mut res);

        res // Return the result
    }
}

fn main() {
    // Example usage
    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    }));

    let result = Solution::inorder_traversal(Some(root));
    println!("Inorder Traversal Result: {:?}", result);
}
