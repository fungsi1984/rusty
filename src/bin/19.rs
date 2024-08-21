use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        remove_nth_from_end_recr(head, n).0
    }
}

fn remove_nth_from_end_recr(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, usize) {
    match head {
        None => (None, 1),
        Some(mut node) => {
            let (prev, num) = remove_nth_from_end_recr(node.next.take(), n);
            if n == num as i32 {
                (prev, num + 1)
            } else {
                node.next = prev;
                (Some(node), num + 1)
            }
        }
    }
}

fn main() {
    // Create the linked list: [1, 2, 3, 4, 5]
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(4)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(5)));

    // Remove the 2nd node from the end (node with value 4)
    let new_head = Solution::remove_nth_from_end(head, 2);

    // Print the modified linked list: [1, 2, 3, 5]
    let mut current = new_head;
    println!("Modified list:");
    while let Some(node) = current {
        println!("{}", node.val);
        current = node.next;
    }
}
