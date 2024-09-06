// Definition for singly-linked list.
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Create a HashSet for efficient lookup of values in nums
        let values_to_remove: std::collections::HashSet<i32> = nums.into_iter().collect();

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

// Function to convert an array into a linked list (ListNode)
fn array_to_linked_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut current = &mut head;

    for &val in arr {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }

    head
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

fn main() {
    // Example input: nums = [1, 2, 3], head = [1, 2, 3, 4, 5]
    let nums = vec![1, 2, 3];
    let head_array = vec![1, 2, 3, 4, 5];

    // Convert array into linked list
    let head = array_to_linked_list(&head_array);

    // Modify the list
    let modified_list = Solution::modified_list(nums, head);

    // Print the modified list
    print_list(modified_list);
}
