// Definition for a binary tree node.
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

use std::cell::RefCell;
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

fn main() {
    // Example usage:
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let k = 1;
    let result = Solution::kth_largest_perfect_subtree(root, k);
    println!("The result is: {}", result);
}
