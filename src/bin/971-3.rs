use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: TreeNodeRef,
    right: TreeNodeRef,
}

impl TreeNode {
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
    pub fn flip_match_voyage(root: TreeNodeRef, voyage: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut i = 0;
        if Self::dfs(&root, &mut i, &voyage, &mut ans) {
            ans
        } else {
            vec![-1]
        }
    }

    fn dfs(root: &TreeNodeRef, i: &mut usize, voyage: &[i32], ans: &mut Vec<i32>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val != voyage[*i] {
                return false;
            }
            *i += 1;

            if *i < voyage.len() && node.left.is_some() && node.left.as_ref().unwrap().borrow().val != voyage[*i] {
                ans.push(node.val); // Flip
                if !Self::dfs(&node.right, i, voyage, ans) || !Self::dfs(&node.left, i, voyage, ans) {
                    return false;
                }
            } else {
                if !Self::dfs(&node.left, i, voyage, ans) || !Self::dfs(&node.right, i, voyage, ans) {
                    return false;
                }
            }
        }
        true
    }
}

fn vec_to_tree(values: Vec<i32>) -> TreeNodeRef {
    if values.is_empty() {
        return None;
    }

    let values = values
        .into_iter()
        .map(|v| if v == -1 { None } else { Some(v) })
        .collect::<Vec<_>>();

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = vec![Rc::clone(&root)];
    let mut i = 1;

    while i < values.len() {
        let node = queue.remove(0); // Directly remove the node without `Some`

        if let Some(Some(val)) = values.get(i) {
            let left = Rc::new(RefCell::new(TreeNode::new(*val)));
            node.borrow_mut().left = Some(Rc::clone(&left));
            queue.push(left);
        }
        i += 1;

        if let Some(Some(val)) = values.get(i) {
            let right = Rc::new(RefCell::new(TreeNode::new(*val)));
            node.borrow_mut().right = Some(Rc::clone(&right));
            queue.push(right);
        }
        i += 1;
    }

    Some(root)
}
fn main() {
    let root = vec_to_tree(vec![1, 2, 3]);
    let voyage = vec![1, 3, 2];
    let result = Solution::flip_match_voyage(root, voyage);
    println!("{:?}", result); // Output should be [1]
}
