use std::collections::HashSet;
use std::ptr;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Create a HashSet for efficient lookup of values in nums
        let values_to_remove: HashSet<i32> = nums.into_iter().collect();

        // Handle the case where the head node needs to be removed
        while let Some(mut node) = head {
            if values_to_remove.contains(&node.val) {
                head = node.next.take();
            } else {
                head = Some(node);
                break;
            }
        }

        // If the list is empty after removing head nodes, return None
        let mut current = head.as_mut()?;

        // Iterate through the list, removing nodes with values in the set
        while let Some(mut next_node) = current.next.take() {
            if values_to_remove.contains(&next_node.val) {
                // Skip the node with the matching value
                current.next = next_node.next.take();
            } else {
                // Keep moving to the next node
                current.next = Some(next_node);
                current = current.next.as_mut().unwrap();
            }
        }

        head
    }
}

struct Solution;
fn main() {
    // Example input: nums = [1, 3], list = 1 -> 2 -> 3 -> 4 -> 5
    let nums = vec![1, 3];
    let mut list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));

    // Modify the list
    let modified_list = Solution::modified_list(nums, list);

    // Print the modified list
    print_list(modified_list);
}

// Helper function to print the linked list
fn print_list(head: Option<Box<ListNode>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next;
    }
    println!("None");
}
