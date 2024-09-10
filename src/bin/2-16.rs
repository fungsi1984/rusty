struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut carry = 0;
        let mut head = None;
        let mut tail = &mut head;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;
            *tail = Some(Box::new(ListNode::new(sum % 10)));
            if let Some(node) = tail {
                tail = &mut node.next;
            }
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        head
    }
}

fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }
    head
}

fn print_linked_list(list: Option<Box<ListNode>>) {
    let mut current = list;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
    println!();
}

struct Solution;
fn main() {
    // Example Test Case
    let l1 = to_linked_list(vec![2, 4, 3]); // Represents 342
    let l2 = to_linked_list(vec![5, 6, 4]); // Represents 465
    let l3 = to_linked_list(vec![1, 4, 2]); // Represents 342
    let l4 = to_linked_list(vec![4, 6, 5]); // Represents 465
    let result = Solution::add_two_numbers(l1, l2); // Should represent 807 (7 -> 0 -> 8)
    let result2 = Solution::add_two_numbers(l3, l4); // Should represent 807 (7 -> 0 -> 8)
    print_linked_list(result); // Expected output: 7 0 8
    print_linked_list(result2); // Expected output: 7 0 8
}
