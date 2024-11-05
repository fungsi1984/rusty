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
        type DepthInfo = [(i32, i32); 100001];
        
        let mut max_depths_by_level = [(-1, -1); 100001]; // max two depths for each level
        let mut node_level_and_depth = max_depths_by_level.clone(); // stores level and depth per node
        
        // Depth-First Search to populate depth arrays
        fn dfs(
            max_depths_by_level: &mut DepthInfo,
            node_level_and_depth: &mut DepthInfo,
            current_node: &Option<Rc<RefCell<TreeNode>>>,
            level: i32
        ) -> i32 {
            // Base case: return -1 if node is None
            let Some(current_node) = current_node else { return -1 };
            let mut node = current_node.borrow_mut();

            // Calculate depth for left and right subtrees
            let left_depth = dfs(max_depths_by_level, node_level_and_depth, &node.left, level + 1);
            let right_depth = dfs(max_depths_by_level, node_level_and_depth, &node.right, level + 1);
            let depth = 1 + left_depth.max(right_depth); // max depth from children + 1
            
            // Update node level and depth information
            node_level_and_depth[node.val as usize] = (level, depth);
            
            // Update max depth values for the current level
            let level_depths = &mut max_depths_by_level[level as usize];
            if depth > level_depths.0 {
                level_depths.1 = level_depths.0;
                level_depths.0 = depth;
            } else {
                level_depths.1 = level_depths.1.max(depth);
            }

            depth
        }

        // Start DFS from the root at level 0
        dfs(&mut max_depths_by_level, &mut node_level_and_depth, &root, 0);

        // Process each query
        queries
            .iter()
            .map(|&query| {
                let (level, depth) = node_level_and_depth[query as usize];
                let (deepest, second_deepest) = max_depths_by_level[level as usize];
                level + if depth < deepest { deepest } else { second_deepest }
            })
            .collect()
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
