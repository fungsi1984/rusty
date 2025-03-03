// Define the ListNode structure
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

// Helper function to get the value of a node or 0 if the node is None
fn get_value(node: &Option<Box<ListNode>>) -> i32 {
    node.as_ref().map_or(0, |x| x.val)
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None; // Result list head
        let mut curr = &mut head; // Mutable reference to the current node in the result list

        let mut carry = 0; // Carry from the previous addition
        while l1.is_some() || l2.is_some() || carry != 0 {
            // Calculate the sum of the current digits and the carry
            let sum = get_value(&l1) + get_value(&l2) + carry;
            carry = sum / 10; // Update the carry

            // Insert a new node with the sum % 10 into the result list
            let sum_node = curr.insert(Box::new(ListNode::new(sum % 10)));
            // Move `curr` to the `next` field of the newly inserted node
            curr = &mut sum_node.next;

            // Move to the next nodes in `l1` and `l2`
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        head // Return the result list
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
    let l1 = to_list(vec![2, 4, 3]); // Represents the number 342
    let l2 = to_list(vec![5, 6, 4]); // Represents the number 465
    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", to_vec(result)); // Output: [7, 0, 8] (represents 807)
}
