// Definition for singly-linked list.
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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }

        let mut dummy_head = Box::new(ListNode::new(0));
        let mut curr: *mut ListNode = &mut *dummy_head; // Raw pointer to the current node
        let mut carry = 0;

        let mut l1_ptr: *const ListNode =
            l1.as_ref().map_or(std::ptr::null(), |node| &**node);
        let mut l2_ptr: *const ListNode =
            l2.as_ref().map_or(std::ptr::null(), |node| &**node);

        while !l1_ptr.is_null() || !l2_ptr.is_null() || carry != 0 {
            let mut v = carry;

            if !l1_ptr.is_null() {
                unsafe {
                    v += (*l1_ptr).val;
                    l1_ptr = (*l1_ptr)
                        .next
                        .as_ref()
                        .map_or(std::ptr::null(), |node| &**node);
                }
            }

            if !l2_ptr.is_null() {
                unsafe {
                    v += (*l2_ptr).val;
                    l2_ptr = (*l2_ptr)
                        .next
                        .as_ref()
                        .map_or(std::ptr::null(), |node| &**node);
                }
            }

            carry = v / 10;
            v %= 10;

            unsafe {
                (*curr).next = Some(Box::new(ListNode::new(v)));
                if let Some(ref mut next_node) = (*curr).next {
                    curr = &mut **next_node; // Move `curr` forward
                }
            }
        }

        dummy_head.next
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