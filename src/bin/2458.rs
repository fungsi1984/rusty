use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;

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

struct Solution;

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

        // Initialize max_depth_left and max_depth_right arrays
        if let Some(root_node) = root {
            max_depth_left[0] = node_heights[&root_node.borrow().val];
            max_depth_right[tour_size - 1] = node_heights[&root_node.borrow().val];
        }

        // Build max_depth_left and max_depth_right arrays
        for i in 1..tour_size {
            max_depth_left[i] = max(max_depth_left[i - 1], node_heights[&euler_tour[i]]);
        }

        for i in (0..tour_size - 1).rev() {
            max_depth_right[i] = max(max_depth_right[i + 1], node_heights[&euler_tour[i]]);
        }

        // Process queries
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
                max(left_max, right_max)
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

fn main() {
    // Build the tree structure
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));

    // Construct the tree by linking nodes
    root.borrow_mut().left = Some(node2.clone());
    root.borrow_mut().right = Some(node3.clone());
    node2.borrow_mut().left = Some(node4.clone());
    node2.borrow_mut().right = Some(node5.clone());
    node3.borrow_mut().right = Some(node6.clone());

    // Queries
    let queries = vec![2, 3, 4];

    // Run tree_queries function and print the results
    let result = Solution::tree_queries(Some(root), queries);
    println!("{:?}", result); // Expected output based on tree structure
}
