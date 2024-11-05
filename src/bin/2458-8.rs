use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution;

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        type D = [(i32, i32); 100001]; 
        let mut ld = [(-1, -1); 100001];
        let mut vld = ld.clone();

        fn dfs(ld: &mut D, vld: &mut D, n: &Option<Rc<RefCell<TreeNode>>>, lvl: i32) -> i32 {
            let Some(n) = n else { return -1 };
            let mut n = n.borrow_mut();
            let d = 1 + dfs(ld, vld, &n.left, lvl + 1).max(dfs(ld, vld, &n.right, lvl + 1));
            vld[n.val as usize] = (lvl, d);
            let m = &mut ld[lvl as usize];
            if d > m.0 {
                m.1 = m.0;
                m.0 = d;
            } else {
                m.1 = m.1.max(d);
            }
            d
        }

        dfs(&mut ld, &mut vld, &root, 0);
        queries.iter().map(|&q| {
            let (lvl, d) = vld[q as usize];
            let (d1, d2) = ld[lvl as usize];
            lvl + if d < d1 { d1 } else { d2 }
        }).collect()
    }
}

fn main() {
    // Create a simple binary tree with values [1, 2, 3, None, 4]
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left_child = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_child = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_right_child = Rc::new(RefCell::new(TreeNode::new(4)));

    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());
    left_child.borrow_mut().right = Some(left_right_child.clone());

    // Define queries
    let queries = vec![1, 2, 3, 4];

    // Get results from tree_queries
    let result = Solution::tree_queries(Some(root), queries);

    // Print the results
    println!("{:?}", result);
}
