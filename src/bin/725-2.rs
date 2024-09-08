#[derive(Debug, PartialEq, Eq)]
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
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut length = 0;
        let mut current = head.as_ref();
        let mut parts = Vec::new();

        while let Some(node) = current {
            length += 1;
            current = node.next.as_ref();
        }

        let (mut base_size, mut extra) = (length / k, length % k);
        let mut current = head;

        for _ in 0..k {
            let mut part_size = base_size + if extra > 0 { 1 } else { 0 };
            let mut dummy = Box::new(ListNode { val: 0, next: None });
            let mut tail = &mut dummy;

            while part_size > 0 {
                tail.next = current.take();
                tail = tail.next.as_mut().unwrap();
                current = tail.next.take();
                part_size -= 1;
            }

            parts.push(dummy.next.take());
            if extra > 0 {
                extra -= 1;
            }
        }

        parts
    }
}

// Helper function to create a linked list from a vector of integers
// fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
//     let mut head = None;
//     let mut current = &mut head;
//     for &value in values.iter().rev() {
//         let new_node = Some(Box::new(ListNode::new(value)));
//         *current = new_node;
//         current = &mut current.as_mut().unwrap().next;
//     }
//     head
// }

// Helper function to create a linked list from a vector of integers
fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    for &value in values.iter() {
        // Removed .rev()
        let new_node = Some(Box::new(ListNode::new(value)));
        *current = new_node;
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

// Helper function to convert a linked list to a vector of integers
fn linked_list_to_vector(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(ref node) = current {
        result.push(node.val);
        current = &node.next;
    }
    result
}

fn main() {
    // Create a linked list from a vector [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    let linked_list = create_linked_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Split the linked list into 3 parts
    let parts = Solution::split_list_to_parts(linked_list, 3);

    // Print each part as a vector of integers for easy visualization
    for (i, part) in parts.iter().enumerate() {
        let part_vec = linked_list_to_vector(part);
        println!("Part {}: {:?}", i + 1, part_vec);
    }
}
