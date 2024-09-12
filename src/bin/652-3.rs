use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// #[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

struct Solution;

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        use std::collections::HashMap;
        fn dfs_collect_duplicate_subtrees(
            root: Option<Rc<RefCell<TreeNode>>>,
            fr: &mut HashMap<Vec<i32>, u16>,
            tres: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Vec<i32> {
            if let Some(rc_node) = root {
                let TreeNode { val, left, right } = &*rc_node.borrow();
                let tree_identifier = [
                    Vec::from([*val]),
                    dfs_collect_duplicate_subtrees(left.clone(), fr, tres),
                    dfs_collect_duplicate_subtrees(right.clone(), fr, tres),
                ]
                .concat();
                if let Some(freq) = fr.get_mut(&tree_identifier) {
                    if *freq == 1 {
                        //collect only when seeing a duplicate for the first time
                        tres.push(Some(rc_node.clone()));
                        *freq += 1;
                    }
                } else {
                    fr.insert(tree_identifier.clone(), 1);
                }
                tree_identifier
            } else {
                vec![-201] //use to identify a null tree arm, according to constraints this is no value
            }
        }
        let mut duplicate_roots = vec![];
        dfs_collect_duplicate_subtrees(root, &mut HashMap::new(), &mut duplicate_roots);
        duplicate_roots
    }
}

// Helper function to build a binary tree from a vector input
fn build_tree(nodes: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nodes.is_empty() || nodes[0] == -1 {
        return None;
    }

    let root = TreeNode::new(nodes[0]);
    let mut q = vec![root.clone()];
    let mut i = 1;

    while i < nodes.len() {
        let curr = q.remove(0);

        // Add left child
        if nodes[i] != -1 {
            let left_node = TreeNode::new(nodes[i]);
            curr.borrow_mut().left = Some(left_node.clone());
            q.push(left_node);
        }
        i += 1;

        // Add right child
        if i < nodes.len() && nodes[i] != -1 {
            let right_node = TreeNode::new(nodes[i]);
            curr.borrow_mut().right = Some(right_node.clone());
            q.push(right_node);
        }
        i += 1;
    }

    Some(root)
}

// Preorder traversal to collect values of a subtree
fn preorder(root: &Option<Rc<RefCell<TreeNode>>>, subtree: &mut Vec<i32>) {
    if let Some(node) = root {
        let node_borrowed = node.borrow();
        subtree.push(node_borrowed.val);
        preorder(&node_borrowed.left, subtree);
        preorder(&node_borrowed.right, subtree);
    }
}

// Helper function to convert TreeNode vector output to a vector of vectors
fn convert_output(duplicates: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for node in duplicates {
        let mut subtree = Vec::new();
        preorder(&node, &mut subtree);
        result.push(subtree);
    }

    result
}

fn main() {
    // Input: root = [1, 2, 3, 4, -1, 2, 4, -1, -1, 4] (-1 represents null)
    let nodes = vec![1, 2, 3, 4, -1, 2, 4, -1, -1, 4];

    let root = build_tree(&nodes);

    let duplicates = Solution::find_duplicate_subtrees(root);

    // Convert the output to desired format
    let output = convert_output(duplicates);

    // Output: [[2, 4], [4]]
    println!("Output: {:?}", output);
}
