use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Definition for singly-linked list.
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

// Implement `Ord` for `ListNode` to allow them to be stored in `BinaryHeap`.
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make it a min-heap
        other.val.cmp(&self.val)
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        let mut heap = BinaryHeap::new();

        // Push the head of each list into the heap
        for list in lists {
            if let Some(node) = list {
                heap.push(node);
            }
        }

        // Continuously extract the smallest element from the heap and add it to the merged list
        while let Some(mut min_node) = heap.pop() {
            if let Some(next) = min_node.next.take() {
                heap.push(next);
            }
            // Use `Option::insert` to set the `next` pointer of the current node
            curr.next.insert(min_node);
            // Safely update `curr` without using `unwrap`
            if let Some(ref mut next_node) = curr.next {
                curr = next_node;
            }
        }

        dummy.next
    }

    // Helper function to convert a vector of vectors into a vector of linked lists
    pub fn vec_to_lists(lists: Vec<Vec<i32>>) -> Vec<Option<Box<ListNode>>> {
        lists
            .into_iter()
            .map(|v| {
                let mut head = None;
                let mut curr = &mut head;
                for val in v {
                    *curr = Some(Box::new(ListNode::new(val)));
                    // Safely update `curr` without using `unwrap`
                    if let Some(ref mut node) = curr {
                        curr = &mut node.next;
                    }
                }
                head
            })
            .collect()
    }

    // Helper function to convert a linked list into a vector (for testing purposes)
    pub fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }
}

fn main() {
    // Example input
    let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];

    // Convert the input into a vector of linked lists
    let linked_lists = Solution::vec_to_lists(lists);

    // Merge the linked lists
    let merged_list = Solution::merge_k_lists(linked_lists);

    // Convert the merged linked list back into a vector
    let result = Solution::list_to_vec(merged_list);

    // Print the result
    println!("{:?}", result); // Output: [1, 1, 2, 3, 4, 4, 5, 6]
}
