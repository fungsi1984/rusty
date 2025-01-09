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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rem = 0;
        let mut nodes = vec![];
        while l1.is_some() || l2.is_some() || rem != 0 {
            let mut v = rem;
            if let Some(n) = l1 {
                v += n.val;
                l1 = n.next;
            }
            if let Some(n) = l2 {
                v += n.val;
                l2 = n.next;
            }
            rem = v / 10;
            nodes.push(ListNode::new(v % 10));
        }

        let mut result = None;
        for mut node in nodes.into_iter().rev() {
            node.next = result;
            result = Some(Box::new(node));
        }
        result

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
