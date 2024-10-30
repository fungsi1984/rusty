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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct T {
    is_perfect: bool,
    sz: i32,
}

impl T {
    fn new(is_perfect: bool, sz: i32) -> Self {
        T { is_perfect, sz }
    }
}

struct Solution;

impl Solution {
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut ans = vec![];
        Self::dfs(root, &mut ans);
        ans.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        if ans.len() < k as usize {
            -1
        } else {
            ans[k as usize - 1]
        }
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) -> T {
        if root.is_none() {
            return T::new(true, 0);
        }

        let node = root.unwrap();
        let left = Self::dfs(node.borrow().left.clone(), ans);
        let right = Self::dfs(node.borrow().right.clone(), ans);

        if left.is_perfect && right.is_perfect && left.sz == right.sz {
            let sz = 1 + left.sz + right.sz;
            ans.push(sz);
            return T::new(true, sz);
        }
        T::new(false, 0)
    }
}

// New function to build a binary tree from a vector
fn build_tree_from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;

    while i < values.len() {
        if let Some(current_node) = queue.pop_front() {
            // Add left child
            if i < values.len() && values[i].is_some() {
                let left_child = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                current_node.borrow_mut().left = Some(Rc::clone(&left_child));
                queue.push_back(left_child);
            }
            i += 1;

            // Add right child
            if i < values.len() && values[i].is_some() {
                let right_child = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                current_node.borrow_mut().right = Some(Rc::clone(&right_child));
                queue.push_back(right_child);
            }
            i += 1;
        }
    }

    Some(root)
}
fn main() {
    // Input: root = [5,3,6,5,2,5,7,1,8,null,null,6,8], k = 2
    let values = vec![
        Some(5),
        Some(3),
        Some(6),
        Some(5),
        Some(2),
        Some(5),
        Some(7),
        Some(1),
        Some(8),
        None,
        None,
        Some(6),
        Some(8),
    ];
    let k = 2;

    // Build the tree from the vector
    let root = build_tree_from_vec(values);
    let result = Solution::kth_largest_perfect_subtree(root, k);

    println!("The result is: {}", result); // Expected Output: 3
}
