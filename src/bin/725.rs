// Definition for singly-linked list.
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
//     for &value in values.iter() {
//         // Removed .rev()
//         let new_node = Some(Box::new(ListNode::new(value)));
//         *current = new_node;
//         current = &mut current.as_mut().unwrap().next;
//     }
//     head
// }


fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &value in values.iter() {
        let new_node = Some(Box::new(ListNode::new(value)));
        
        *current = new_node;
        if let Some(ref mut node) = current {
            current = &mut node.next;
        }
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
    // Input: head = [1,2,3,4,5,6,7,8,9,10], k = 3
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let root = create_linked_list(values);
    let k = 3;

    // Split the list into k parts
    let result = Solution::split_list_to_parts(root, k);

    // Output the result
    print!("Output: [");
    for (i, part) in result.iter().enumerate() {
        let part_vec = linked_list_to_vector(part);
        print!("[");
        for (j, val) in part_vec.iter().enumerate() {
            print!("{}", val);
            if j < part_vec.len() - 1 {
                print!(",");
            }
        }
        print!("]");
        if i < k as usize - 1 {
            print!(",");
        }
    }
    println!("]");
}
