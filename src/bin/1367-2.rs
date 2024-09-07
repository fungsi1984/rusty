use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    // #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Definition for a binary tree node.
// #[derive(PartialEq, Eq, Clone)]
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
        type OptListNode = Option<Box<ListNode>>;

        fn _is_sub_path(head: &OptListNode, root: &OptTreeNode) -> Option<bool> {
            if head.is_none() {
                return Some(true);
            }
            if root.is_none() {
                return Some(false);
            }
            let root_ref = root.as_ref()?.borrow();
            let v = dfs(head, root)?
                || _is_sub_path(head, &root_ref.left)?
                || _is_sub_path(head, &root_ref.right)?;
            Some(v)
        }

        fn dfs(head: &OptListNode, root: &OptTreeNode) -> Option<bool> {
            if head.is_none() {
                return Some(true);
            }
            if root.is_none() {
                return Some(false);
            }
            let root_ref = root.as_ref()?.borrow();
            let v = head.as_ref()?.val == root_ref.val
                && (dfs(&head.as_ref()?.next, &root_ref.left)?
                    || dfs(&head.as_ref()?.next, &root_ref.right)?);
            Some(v)
        }

        _is_sub_path(&head, &root).unwrap_or(false)
    }
}
// Helper function to create a linked list from a vector.
fn create_linked_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

// Helper function to create a binary tree from a vector.
fn create_binary_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = vec![root.clone()];
    let mut i = 1;

    while i < vals.len() {
        let current = queue.remove(0);

        if i < vals.len() && vals[i].is_some() {
            let left = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
            current.borrow_mut().left = Some(left.clone());
            queue.push(left);
        }
        i += 1;

        if i < vals.len() && vals[i].is_some() {
            let right = Rc::new(RefCell::new(TreeNode::new(vals[i].unwrap())));
            current.borrow_mut().right = Some(right.clone());
            queue.push(right);
        }
        i += 1;
    }

    Some(root)
}

fn main() {
    // Input linked list: [4, 2, 8]
    let head = create_linked_list(vec![4, 2, 8]);

    // Input binary tree: [1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]
    let root = create_binary_tree(vec![
        Some(1),
        Some(4),
        Some(4),
        None,
        Some(2),
        Some(2),
        None,
        Some(1),
        None,
        Some(6),
        Some(8),
        None,
        None,
        None,
        None,
        Some(1),
        Some(3),
    ]);

    // Output: true
    let result = Solution::is_sub_path(head, root);
    println!("Output: {}", result); // Expected output: true
}
