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

// Convert Vec<Option<i32>> into a binary tree (level-order)
fn vec_to_tree_node(data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() || data[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(data[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;

    while i < data.len() {
        if let Some(current) = queue.pop_front() {
            if i < data.len() && data[i].is_some() {
                let left_node = Rc::new(RefCell::new(TreeNode::new(data[i].unwrap())));
                current.borrow_mut().left = Some(left_node.clone());
                queue.push_back(left_node);
            }
            i += 1;

            if i < data.len() && data[i].is_some() {
                let right_node = Rc::new(RefCell::new(TreeNode::new(data[i].unwrap())));
                current.borrow_mut().right = Some(right_node.clone());
                queue.push_back(right_node);
            }
            i += 1;
        }
    }

    Some(root)
}

// Convert binary tree into Vec<Option<i32>> (level-order)
fn tree_node_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node_option) = queue.pop_front() {
        if let Some(node) = node_option {
            let node_ref = node.borrow();
            result.push(Some(node_ref.val));
            queue.push_back(node_ref.left.clone());
            queue.push_back(node_ref.right.clone());
        } else {
            result.push(None);
        }
    }

    // Remove trailing None values for cleaner output
    while result.last() == Some(&None) {
        result.pop();
    }

    result
}

struct Solution;
fn main() {
    let root_values = vec![5, 8, 9, 2, 1, 3, 7, 4, 6];
    let root: Vec<Option<i32>> = root_values.into_iter().map(Some).collect();

    // Build tree from the vector
    let root_node = vec_to_tree_node(root);

    // Queries
    let queries = vec![3, 2, 4, 8];

    // Execute tree_queries and print results
    let result = Solution::tree_queries(root_node, queries);
    println!("Output: {:?}", result); // Expected: [3, 2, 3, 2]
}