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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        if root.is_none() {
            return -1;
        }

        let mut node_queue = VecDeque::new();
        node_queue.push_back(root.unwrap());

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
    }
}

struct Solution;

fn main() {
    // Example input: root = [5, 8, 9, 2, 1, 3, 7, 4, 6], k = 2
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));

    let k = 2;
    let result = Solution::kth_largest_level_sum(root, k);
    println!("Output: {}", result);  // Expected output: 13
}