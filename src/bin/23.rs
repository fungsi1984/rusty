use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Convert a vector to a linked list
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    // for val in vec {
    //     tail.next = Some(Box::new(ListNode::new(val)));
    //     tail = tail.next.as_mut().unwrap();
    // }

    for val in vec {
        tail.next = Some(Box::new(ListNode::new(val)));
        // Safely advance tail using match
        tail = match tail.next.as_mut() {
            Some(node) => node,
            None => panic!("Unexpected None value while advancing tail"),
        };
    }
    dummy.next
}

// Convert a linked list to a vector
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

struct HeapNode(Box<ListNode>);

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.val == other.0.val
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.val.cmp(&self.0.val)
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for list in lists {
            if let Some(node) = list {
                heap.push(HeapNode(node));
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let Some(HeapNode(mut node)) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(HeapNode(next));
            }
            tail.next = Some(node);
            // tail = tail.next.as_mut().unwrap();
            tail = match tail.next.as_mut() {
                Some(node) => node,
                None => panic!("some error"),
            };
        }

        dummy.next
    }
}

fn main() {
    // Input: lists = [[1,4,5],[1,3,4],[2,6]]
    let lists_input = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];

    let lists: Vec<Option<Box<ListNode>>> = lists_input
        .into_iter()
        .map(|vec| vec_to_list(vec))
        .collect();

    let merged_list = Solution::merge_k_lists(lists);
    let merged_vec = list_to_vec(merged_list);

    println!("{:?}", merged_vec); // Output: [1, 1, 2, 3, 4, 4, 5, 6]
}
