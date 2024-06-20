use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type OptNode = Option<Rc<RefCell<TreeNode>>>;
        fn validate(n: &OptNode, min: &OptNode, max: &OptNode) -> Option<bool> {
            if n.is_none() {
                return Some(true);
            }
            let val: i32 = n.as_ref()?.borrow().val;
            if max.is_some() && max.as_ref()?.borrow().val <= val {
                return Some(false);
            }
            if min.is_some() && min.as_ref()?.borrow().val >= val {
                return Some(false);
            }
            let v: bool = validate(&n.as_ref()?.borrow().left, min, n)? && validate(&n.as_ref()?.borrow().right, n, max)?;
            Some(v)
        }
        validate(&root, &None, &None).unwrap_or_default()
    }
}

fn from_vec(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode {
        val: values[0],
        left: None,
        right: None,
    }));

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while !queue.is_empty() && i < values.len() {
        let node = queue.pop_front().unwrap();
        let mut borrow = node.borrow_mut();

        if i < values.len() {
            borrow.left = Some(Rc::new(RefCell::new(TreeNode {
                val: values[i],
                left: None,
                right: None,
            })));
            queue.push_back(borrow.left.as_ref().unwrap().clone());
            i += 1;
        }

        if i < values.len() {
            borrow.right = Some(Rc::new(RefCell::new(TreeNode {
                val: values[i],
                left: None,
                right: None,
            })));
            queue.push_back(borrow.right.as_ref().unwrap().clone());
            i += 1;
        }
    }

    Some(root)
}
struct Solution;

// without function from_vec 
/*
// fn main() {
    // Create a valid binary search tree
    let root = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));

    // Check if the binary tree is a valid BST
    let is_valid = Solution::is_valid_bst(Some(root));
    println!("Is the binary tree a valid BST? {}", is_valid); // Output: Is the binary tree a valid BST? true
}

*/

fn main() {
    let input = vec![2, 1, 3];
    let root = from_vec(&input);
    let is_valid = Solution::is_valid_bst(root);
    println!("Is the binary tree a valid BST? {}", is_valid); // Output: Is the binary tree a valid BST? true
}

