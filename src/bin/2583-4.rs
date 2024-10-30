use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::rc::Rc;

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

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        if let Some(root_rc) = root {
            let mut node_queue = VecDeque::new();
            node_queue.push_back(root_rc);

            let mut min_heap = BinaryHeap::new();
            let k = k as usize;

            while !node_queue.is_empty() {
                let q_size = node_queue.len();
                let mut level_sum: i64 = 0;

                for _ in 0..q_size {
                    if let Some(node_rc) = node_queue.pop_front() {
                        let node = node_rc.borrow();
                        level_sum += node.val as i64;

                        if let Some(left) = node.left.clone() {
                            node_queue.push_back(left);
                        }
                        if let Some(right) = node.right.clone() {
                            node_queue.push_back(right);
                        }
                    }
                }

                min_heap.push(Reverse(level_sum));
                if min_heap.len() > k {
                    min_heap.pop();
                }
            }

            if min_heap.len() < k {
                -1
            } else {
                min_heap.peek().map_or(-1, |Reverse(val)| *val)
            }
        } else {
            -1
        }
    }
}

fn from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    // Use pattern matching instead of unwrap
    let root = if let Some(v) = values[0] {
        Rc::new(RefCell::new(TreeNode::new(v)))
    } else {
        return None;
    };

    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < values.len() {
        if let Some(val) = queue.pop_front() {
            if i < values.len() {
                if let Some(v) = values[i] {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(v)));
                    val.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                }
                i += 1;
            }

            if i < values.len() {
                if let Some(v) = values[i] {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(v)));
                    val.borrow_mut().right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                }
                i += 1;
            }
        }
    }

    Some(root)
}

struct Solution;

fn main() {
    // Example input: root = [5, 8, 9, 2, 1, 3, 7, 4, 6], k = 2
    let tree_vec = vec![
        Some(5),
        Some(8),
        Some(9),
        Some(2),
        Some(1),
        Some(3),
        Some(7),
        Some(4),
        Some(6),
    ];

    let root = from_vec(tree_vec);
    let k = 2;

    let result = Solution::kth_largest_level_sum(root, k);
    println!("Output: {}", result); // Expected output: 13
}
