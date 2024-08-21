#[derive(PartialEq, Eq, Clone, Debug)]
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
    // Function to remove the Nth node from the end
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        remove_nth_from_end_recr(head, n).0
    }
}

// Recursive helper function to remove the Nth node
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

// Helper function to convert an array into a linked list
// fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
//     let mut head = None;
//     let mut current = &mut head;

//     for &value in arr.iter() {
//         *current = Some(Box::new(ListNode::new(value)));
//         current = &mut current.as_mut().unwrap().next;
//     }

//     head
// }

// fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
//     let mut head = None;
//     let mut current = &mut head;

//     for &value in arr.iter() {
//         match current {
//             Some(node) => current = &mut node.next,
//             None => *current = Some(Box::new(ListNode::new(value))),
//         }
//     }

//     head
// }

// Helper function to convert an array into a linked list
// fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
//     let mut head: Option<Box<ListNode>> = None;
//     let mut current = &mut head;

//     for &value in arr.iter() {
//         match current {
//             Some(node) => current = &mut node.as_mut().next,
//             None => *current = Some(Box::new(ListNode::new(value))),
//         }
//     }

//     head
// }

fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut current = &mut head;

    for &value in arr.iter() {
        *current = Some(Box::new(ListNode::new(value)));
        if let Some(node) = current {
            current = &mut node.next;
        }
    }

    head
}

// Helper function to convert a linked list back into an array
fn linked_list_to_array(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

fn main() {
    let head_array = vec![1, 2, 3, 4, 5];
    let n = 2;

    // Convert array to linked list
    let head = array_to_linked_list(head_array);

    // Remove the Nth node from the end
    let modified_head = Solution::remove_nth_from_end(head, n);

    // Convert linked list back to array
    let result_array = linked_list_to_array(modified_head);

    // Output the result
    println!("{:?}", result_array);
}
