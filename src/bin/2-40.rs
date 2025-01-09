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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_recursive(l1, l2, 0)
    }

    fn add_two_numbers_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        // Base case: if both lists are None and there is no carry, return None
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        // Calculate the sum of the current nodes and the carry
        let x = if let Some(ref node) = l1 { node.val } else { 0 };
        let y = if let Some(ref node) = l2 { node.val } else { 0 };
        let sum = carry + x + y;

        // Create a new node with the value of sum % 10
        let mut new_node = Box::new(ListNode::new(sum % 10));

        // Recursively call the function for the next nodes
        new_node.next = Self::add_two_numbers_recursive(
            if let Some(node) = l1 { node.next } else { None },
            if let Some(node) = l2 { node.next } else { None },
            sum / 10,
        );

        Some(new_node)
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
