// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1 == None && l2 == None {
            return None;
        }
        let mut depth = 0;
        let (mut next1, mut next2) = (l1, l2);
        let mut s = 0;
        let mut ret = None;
        let mut carry = 0;
        while next1 != None || next2 != None || carry > 0 {
            let mut v = carry;
            if let Some(node) = next1 {
                v += node.val;
                next1 = node.next;
            }
            if let Some(node) = next2 {
                v += node.val;
                next2 = node.next;
            }
            carry = v / 10;
            set(&mut ret, v % 10);
            depth += 1;
        }
        ret
    }
}
fn set(node: &mut Option<Box<ListNode>>, val: i32) {
    match node {
        Some(v) => {
            set(&mut v.next, val);
        }
        None => {
            *node = Some(Box::new(ListNode {
                val: val,
                next: None,
            }));
        }
    }
}

// Helper functions for testing
fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = list;
    while let Some(node) = current {
        vec.push(node.val);
        current = node.next;
    }
    vec
}

struct Solution;
fn main() {
    let l1 = to_linked_list(vec![2, 4, 3]);
    let l2 = to_linked_list(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", to_vec(result)); // Output should be [7, 0, 8]
}
