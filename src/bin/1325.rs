use std::cell::RefCell;
use std::rc::Rc;
// Alias for TreeNode wrapped in Rc and RefCell
type Node = Option<Rc<RefCell<TreeNode>>>;

// Define the TreeNode struct
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
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
    pub fn remove_leaf_nodes(root: Node, target: i32) -> Node {
        let rc_refcell = root?;
        {
            let mut root_mut = rc_refcell.borrow_mut();
            root_mut.left = Self::remove_leaf_nodes(root_mut.left.take(), target);
            root_mut.right = Self::remove_leaf_nodes(root_mut.right.take(), target);
        } // Drop RefMut

        {
            let root = rc_refcell.borrow();
            if root.left.is_none() && root.right.is_none() && root.val == target {
                return None;
            }
        } // Drop Ref

        Some(rc_refcell)
    }
}

// Example usage
fn main() {
    // Construct a sample tree
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    // Call the remove_leaf_nodes function
    let modified_tree = Solution::remove_leaf_nodes(root, 2);

    // Print the modified tree (in this example, just print the root value)
    if let Some(rc_refcell) = modified_tree {
        let root = rc_refcell.borrow();
        println!("Root value after removing leaf nodes: {}", root.val);
    } else {
        println!("Tree is empty after removing leaf nodes");
    }
}

#[test]
fn test_remove_leaf_nodes() {
    // Construct a sample tree for testing
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    // Call the remove_leaf_nodes function for testing
    let modified_tree = Solution::remove_leaf_nodes(root, 2);

    // Assert that the modified tree is as expected
    if let Some(rc_refcell) = modified_tree {
        let root = rc_refcell.borrow();
        assert_eq!(root.val, 1);
        assert!(root.left.is_none());
        assert!(root.right.is_some());
    } else {
        panic!("Test failed: Tree is empty after removing leaf nodes");
    }
}
