use std::cell::RefCell;
use std::rc::Rc;

// TreeNode definition
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
    // Function to check flip equivalence between two binary trees
    fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(r1), Some(r2)) => {
                let r1 = r1.borrow();
                let r2 = r2.borrow();

                if r1.val != r2.val {
                    return false;
                }

                let no_swap = Solution::flip_equiv(r1.left.clone(), r2.left.clone())
                    && Solution::flip_equiv(r1.right.clone(), r2.right.clone());

                let swap = Solution::flip_equiv(r1.left.clone(), r2.right.clone())
                    && Solution::flip_equiv(r1.right.clone(), r2.left.clone());

                no_swap || swap
            }
        }
    }
}

// Function to convert a vector into a binary tree
fn vec_to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() || vec[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
    let mut nodes = vec![root.clone()];
    let mut i = 1;

    while i < vec.len() {
        // Get the current node to attach children to
        let current_node = nodes.remove(0);

        // Add the left child
        if i < vec.len() {
            if let Some(Some(left_val)) = vec.get(i) {
                let left_node = Rc::new(RefCell::new(TreeNode::new(*left_val)));
                current_node.borrow_mut().left = Some(left_node.clone());
                nodes.push(left_node);
            }
            i += 1;
        }

        // Add the right child
        if i < vec.len() {
            if let Some(Some(right_val)) = vec.get(i) {
                let right_node = Rc::new(RefCell::new(TreeNode::new(*right_val)));
                current_node.borrow_mut().right = Some(right_node.clone());
                nodes.push(right_node);
            }
            i += 1;
        }
    }

    Some(root)
}


fn main() {
    // Tree 1: [1,2,3,4,5,6,null,null,null,7,8]
    let root1 = vec_to_tree(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        Some(7),
        Some(8),
    ]);

    // Tree 2: [1,3,2,null,6,4,5,null,null,null,null,8,7]
    let root2 = vec_to_tree(vec![
        Some(1),
        Some(3),
        Some(2),
        None,
        Some(6),
        Some(4),
        Some(5),
        None,
        None,
        None,
        None,
        Some(8),
        Some(7),
    ]);

    let result = Solution::flip_equiv(root1, root2);
    println!("Are the two trees flip equivalent? {}", result);
}
