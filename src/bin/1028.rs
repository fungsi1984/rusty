use std::cell::RefCell;
use std::rc::Rc;

// TreeNode definition
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(x: i32) -> Self {
        TreeNode {
            val: x,
            left: None,
            right: None,
        }
    }
}

// Solution struct to recover tree from preorder traversal
struct Solution;

impl Solution {
    pub fn recover_from_preorder(
        traversal: String,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut i = 0;
        Solution::recover_from_preorder_helper(&traversal, 0, &mut i)
    }

    fn recover_from_preorder_helper(
        traversal: &str,
        depth: usize,
        i: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut n_dashes = 0;

        // Count the number of dashes to determine the depth
        while *i + n_dashes < traversal.len()
            && &traversal[*i + n_dashes..*i + n_dashes + 1] == "-"
        {
            n_dashes += 1;
        }

        if n_dashes != depth {
            return None;
        }

        *i += n_dashes;

        // Parse the node value
        let start = *i;
        while *i < traversal.len()
            && traversal.chars().nth(*i).unwrap().is_digit(10)
        {
            *i += 1;
        }

        let node_value = traversal[start..*i].parse::<i32>().unwrap();

        // Create the new tree node
        let node = Rc::new(RefCell::new(TreeNode::new(node_value)));

        // Recursively recover the left and right subtrees
        node.borrow_mut().left =
            Solution::recover_from_preorder_helper(traversal, depth + 1, i);
        node.borrow_mut().right =
            Solution::recover_from_preorder_helper(traversal, depth + 1, i);

        Some(node)
    }
}

fn preorder_traversal(
    root: Option<Rc<RefCell<TreeNode>>>,
    result: &mut Vec<i32>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        // Visit the root node
        result.push(node.val);

        // Visit the left subtree
        preorder_traversal(node.left.clone(), result);

        // Visit the right subtree
        preorder_traversal(node.right.clone(), result);
    }
}

fn main() {
    // The preorder traversal string where '-' represents the depth
    let traversal = "1-2--3--4-5--6--7".to_string();

    // Recover the binary tree from the traversal string
    let root = Solution::recover_from_preorder(traversal.clone());

    // Vector to store the preorder traversal result
    let mut result = Vec::new();

    // Perform preorder traversal to collect the node values
    preorder_traversal(root, &mut result);

    // Output the result in the desired format
    println!("Input: traversal = \"{}\"", traversal);
    print!("Output: [");
    for (i, &val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(",");
        }
    }
    println!("]");
}
