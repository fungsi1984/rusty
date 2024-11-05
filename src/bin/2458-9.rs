use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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


// Helper function to build a tree from level-order representation
fn vec_to_tree_node(data: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() || data[0] == -1 {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(data[0])));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < data.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if data[i] != -1 {
                let left_child = Rc::new(RefCell::new(TreeNode::new(data[i])));
                current.borrow_mut().left = Some(left_child.clone());
                queue.push_back(left_child);
            }
            i += 1;

            // Right child
            if i < data.len() && data[i] != -1 {
                let right_child = Rc::new(RefCell::new(TreeNode::new(data[i])));
                current.borrow_mut().right = Some(right_child.clone());
                queue.push_back(right_child);
            }
            i += 1;
        }
    }

    Some(root)
}

struct Solution;
fn main() {
    let root_values = vec![5, 8, 9, 2, 1, 3, 7, 4, 6];
    let root_node = vec_to_tree_node(root_values);

    // Queries
    let queries = vec![3, 2, 4, 8];

    // Execute tree_queries and print results
    let result = Solution::tree_queries(root_node, queries);

    println!("Output: {:?}", result); // Expected output: [3, 2, 3, 2]
}