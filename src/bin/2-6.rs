// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {


    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let x = l1.as_ref().map_or(0, |node| node.val);
            let y = l2.as_ref().map_or(0, |node| node.val);

            let sum = x + y + carry;
            carry = sum / 10;

            let digit = sum % 10;
            current.next = Some(Box::new(ListNode::new(digit)));

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
            current = current.next.as_mut()?;
        }

        dummy.next
    }
}

fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    for &val in &arr {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut()?;
    }

    dummy.next
}

fn main() {
    let arr1 = vec![2, 4, 3];
    let l1 = array_to_linked_list(arr1);

    let arr2 = vec![5, 6, 4];
    let l2 = array_to_linked_list(arr2);

    let result = Solution::add_two_numbers(l1, l2);

    let mut current = result;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
    println!();
}
