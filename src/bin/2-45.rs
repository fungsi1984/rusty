// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = carry + l1.as_ref().map_or(0, |node| node.val) + l2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;
            current = current.next.insert(Box::new(ListNode::new(sum % 10)));
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy_head.next
    }
}

// Helper function to create a linked list from a vector
fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

// Helper function to convert a linked list to a vector
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
    println!("{:?}", to_vec(result)); // Output: [7, 0, 8]
}
