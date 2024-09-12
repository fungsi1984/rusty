use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
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
        let mut map: HashMap<String, Vec<Rc<RefCell<TreeNode>>>> = HashMap::new();
        let mut dups = Vec::new();
        Solution::serialize(&root, &mut map);
        for nodes in map.values() {
            if nodes.len() > 1 {
                dups.push(Some(nodes[0].clone()));
            }
        }
        dups
    }

    fn serialize(
        node: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<String, Vec<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        if let Some(n) = node {
            let n_borrowed = n.borrow();
            let s = format!(
                "({}){}({})",
                Solution::serialize(&n_borrowed.left, map),
                n_borrowed.val,
                Solution::serialize(&n_borrowed.right, map)
            );
            map.entry(s.clone())
                .or_insert_with(Vec::new)
                .push(n.clone());
            s
        } else {
            "".to_string()
        }
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
