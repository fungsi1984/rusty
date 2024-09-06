struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn add_next(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: bool,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && !carry {
        return None;
    }

    let (val1, next1) = match l1 {
        Some(node) => (node.val, node.next),
        None => (0, None),
    };

    let (val2, next2) = match l2 {
        Some(node) => (node.val, node.next),
        None => (0, None),
    };

    let sum = val1 + val2 + if carry { 1 } else { 0 };
    let digit = sum % 10;
    let new_carry = sum >= 10;

    let mut new_node = ListNode::new(digit);
    new_node.next = add_next(next1, next2, new_carry);

    Some(Box::new(new_node))
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        add_next(l1, l2, false)
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
