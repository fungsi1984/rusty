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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        while (l1 != None || l2 != None || carry != 0) {
            let val = match l1 {
                Some(n) => {
                    l1 = n.next;
                    n.val
                }
                None => 0,
            } + match l2 {
                Some(n) => {
                    l2 = n.next;
                    n.val
                }
                None => 0,
            } + carry;
            carry = val / 10;
            tail.next = Some(Box::new(ListNode::new(val % 10)));
            // tail = tail.next.as_mut().unwrap();
            if let Some(ref mut next_node) = tail.next {
                tail = next_node;
            }
        }
        head.next
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
