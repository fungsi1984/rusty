use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn add_leaf(
            node: &Option<Rc<RefCell<TreeNode>>>,
            leaves: &mut Vec<Rc<RefCell<TreeNode>>>,
            roots: &mut HashMap<i32, Rc<RefCell<TreeNode>>>,
            unique_vals: &mut HashSet<i32>,
        ) {
            if let Some(ref n) = node {
                let val = n.borrow().val;
                unique_vals.insert(val);
                if roots.contains_key(&val) {
                    leaves.push(Rc::clone(n));
                }
            }
        }

        fn valid_bst(
            node: Option<Rc<RefCell<TreeNode>>>,
            min_val: Option<i32>,
            max_val: Option<i32>,
            visited: &mut HashSet<i32>,
        ) -> bool {
            if let Some(n) = node {
                let val = n.borrow().val;
                if !visited.insert(val)
                    || (min_val.is_some() && val <= min_val.unwrap())
                    || (max_val.is_some() && val >= max_val.unwrap())
                {
                    return false;
                }
                valid_bst(n.borrow().left.clone(), min_val, Some(val), visited)
                    && valid_bst(n.borrow().right.clone(), Some(val), max_val, visited)
            } else {
                true
            }
        }

        let mut unique_vals = HashSet::new();
        let mut roots = HashMap::new();
        let mut leaves = Vec::new();

        // Insert trees into roots map and add unique values
        for tree in &trees {
            if let Some(ref root) = tree {
                let val = root.borrow().val;
                roots.insert(val, Rc::clone(root));
                unique_vals.insert(val);
            }
        }

        // Collect leaves and add unique values
        for tree in &trees {
            if let Some(ref root) = tree {
                let val = root.borrow().val;
                unique_vals.insert(val);
                add_leaf(&root.borrow().left, &mut leaves, &mut roots, &mut unique_vals);
                add_leaf(&root.borrow().right, &mut leaves, &mut roots, &mut unique_vals);
            }
        }

        // Merge leaves into roots if valid BST candidates
        for leaf in leaves {
            let val = leaf.borrow().val;
            if let Some(root) = roots.get(&val) {
                let mut leaf_borrow = leaf.borrow_mut();
                leaf_borrow.left = root.borrow().left.clone();
                leaf_borrow.right = root.borrow().right.clone();
                roots.remove(&val);
            }
        }

        // Validate and check for single root in final tree
        let root_node = roots.values().next().cloned()?;
        let mut visited = HashSet::new();
        if roots.len() == 1
            && valid_bst(Some(root_node.clone()), None, None, &mut visited)
            && visited.len() == unique_vals.len()
        {
            Some(root_node)
        } else {
            None
        }
    }
}


fn main() {
    // Construct example trees:
    let tree1 = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    }));
    
    let tree2 = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    }));

    let tree3 = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    }));

    // Put the trees into a vector and call can_merge
    let trees = vec![Some(tree1), Some(tree2), Some(tree3)];
    let merged_tree = Solution::can_merge(trees);

    // Output the result
    match merged_tree {
        Some(root) => println!("Merged tree root value: {:?}", root.borrow().val),
        None => println!("Cannot merge the trees into a valid BST"),
    }
}