use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut euler_tour = Vec::new();
        let mut node_heights = HashMap::new();
        let mut first_occurrence = HashMap::new();
        let mut last_occurrence = HashMap::new();

        // Build Euler tour and populate node information
        Solution::dfs(
            root.clone(),
            0,
            &mut euler_tour,
            &mut node_heights,
            &mut first_occurrence,
            &mut last_occurrence,
        );

        let tour_size = euler_tour.len();
        let mut max_depth_left = vec![0; tour_size];
        let mut max_depth_right = vec![0; tour_size];

        if let Some(root_node) = root {
            max_depth_left[0] = node_heights[&root_node.borrow().val];
            max_depth_right[tour_size - 1] = node_heights[&root_node.borrow().val];
        }

        // Fill max_depth_left and max_depth_right arrays
        for i in 1..tour_size {
            max_depth_left[i] = max_depth_left[i - 1].max(node_heights[&euler_tour[i]]);
        }
        for i in (0..tour_size - 1).rev() {
            max_depth_right[i] = max_depth_right[i + 1].max(node_heights[&euler_tour[i]]);
        }

        // Process queries to get results
        queries
            .iter()
            .map(|&query_node| {
                let left_max = if first_occurrence[&query_node] > 0 {
                    max_depth_left[first_occurrence[&query_node] - 1]
                } else {
                    0
                };
                let right_max = if last_occurrence[&query_node] < tour_size - 1 {
                    max_depth_right[last_occurrence[&query_node] + 1]
                } else {
                    0
                };
                left_max.max(right_max)
            })
            .collect()
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        height: i32,
        euler_tour: &mut Vec<i32>,
        node_heights: &mut HashMap<i32, i32>,
        first_occurrence: &mut HashMap<i32, usize>,
        last_occurrence: &mut HashMap<i32, usize>,
    ) {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();
            let val = node_ref.val;
            node_heights.insert(val, height);
            first_occurrence.entry(val).or_insert(euler_tour.len());
            euler_tour.push(val);

            // Recurse on left and right children
            Solution::dfs(
                node_ref.left.clone(),
                height + 1,
                euler_tour,
                node_heights,
                first_occurrence,
                last_occurrence,
            );
            Solution::dfs(
                node_ref.right.clone(),
                height + 1,
                euler_tour,
                node_heights,
                first_occurrence,
                last_occurrence,
            );

            last_occurrence.insert(val, euler_tour.len());
            euler_tour.push(val);
        }
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
