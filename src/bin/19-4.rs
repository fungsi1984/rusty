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

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Create a dummy node to simplify edge cases (e.g., removing the head)
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        
        // Calculate the length of the list
        let mut len = 0;
        let mut current = &dummy.as_ref().unwrap().next; // Immutable borrow
        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }

        // If `n` is greater than the length, return the original list
        if n > len {
            return dummy.and_then(|d| d.next);
        }

        // Move `slow` to the node before the one to be removed
        let mut slow = &mut dummy; // Mutable borrow
        for _ in 0..(len - n) {
            if let Some(slow_node) = slow {
                slow = &mut slow_node.next;
            }
        }

        // Remove the nth node from the end
        if let Some(slow_node) = slow {
            let next = slow_node.next.take(); // Remove the nth node
            slow_node.next = next.and_then(|node| node.next); // Skip the removed node
        }

        // Return the list without the dummy node
        dummy.and_then(|d| d.next)
    }
}

// Helper function to create a linked list from a vector
fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = current.take(); // Take the current list and set `current` to `None`
        current.insert(Box::new(node)); // Insert the new node as the head of the list
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

// Test the function
fn main() {
    let head = to_list(vec![1, 2, 3, 4, 5]); // Create a linked list: 1 -> 2 -> 3 -> 4 -> 5
    let n = 2;
    let result = Solution::remove_nth_from_end(head, n);
    println!("{:?}", to_vec(result)); // Output: [1, 2, 3, 5]
}