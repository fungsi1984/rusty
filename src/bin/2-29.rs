// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        let mut carry = 0;

        let (mut l1, mut l2) = (l1, l2);

        // while l1.is_some() || l2.is_some() || carry != 0 {
        while (l1 != None || l2 != None || carry != 0) {
            let x = match &l1 {
                Some(node) => node.val,
                None => 0,
            };

            let y = match &l2 {
                Some(node) => node.val,
                None => 0,
            };

            let sum = carry + x + y;
            carry = sum / 10;

            current.next = Some(Box::new(ListNode::new(sum % 10)));
            // current = current.next.as_mut().unwrap();

            // Use pattern matching instead of unwrap
            if let Some(ref mut next_node) = current.next {
                current = next_node;
            }

            // Move to the next nodes
            l1 = match l1 {
                Some(node) => node.next,
                None => None,
            };
            l2 = match l2 {
                Some(node) => node.next,
                None => None,
            };
        }

        dummy_head.next
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

fn main() {
    let l1 = to_linked_list(vec![2, 4, 3]);
    let l2 = to_linked_list(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", to_vec(result)); // Output should be [7, 0, 8]
}
