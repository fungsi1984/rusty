use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;
impl Solution {
    fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type OptNode = Option<Rc<RefCell<TreeNode>>>;
        fn validate(n: &OptNode, min: &OptNode, max: &OptNode) -> Option<bool> {
            if n.is_none() {
                return Some(true);
            }
            let val = n.as_ref()?.borrow().val;
            if max.is_some() && max.as_ref()?.borrow().val <= val {
                return Some(false);
            }
            if min.is_some() && min.as_ref()?.borrow().val >= val {
                return Some(false);
            }
            let v = validate(&n.as_ref()?.borrow().left, min, n)?
                && validate(&n.as_ref()?.borrow().right, n, max)?;
            Some(v)
        }
        validate(&root, &None, &None).unwrap_or_default()
    }
}

fn from_vec(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode {
        val: values[0].unwrap(),
        left: None,
        right: None,
    }));

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while !queue.is_empty() && i < values.len() {
        let node = queue.pop_front().unwrap();
        let mut borrow = node.borrow_mut();

        if i < values.len() && values[i].is_some() {
            borrow.left = Some(Rc::new(RefCell::new(TreeNode {
                val: values[i].unwrap(),
                left: None,
                right: None,
            })));
            queue.push_back(borrow.left.as_ref().unwrap().clone());
            i += 1;
        } else {
            i += 1;
        }

        if i < values.len() && values[i].is_some() {
            borrow.right = Some(Rc::new(RefCell::new(TreeNode {
                val: values[i].unwrap(),
                left: None,
                right: None,
            })));
            queue.push_back(borrow.right.as_ref().unwrap().clone());
            i += 1;
        } else {
            i += 1;
        }
    }

    Some(root)
}

fn main() {
    let input = vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)];
    let root = from_vec(&input);
    let is_valid = Solution::is_valid_bst(root);
    println!("Is the binary tree a valid BST? {}", is_valid); // Output: Is the binary tree a valid BST? true
}
