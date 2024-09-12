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

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        calc_sum(l1, l2, 0)
    }
}

fn calc_sum(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let mut current_node = Box::new(ListNode::new(0));
    let first_digit = l1.as_ref().map_or(0, |node| node.val);
    let second_digit = l2.as_ref().map_or(0, |node| node.val);
    let current_val = first_digit + second_digit + carry;
    current_node.val = current_val % 10;

    current_node.next = calc_sum(
        l1.and_then(|node| node.next),
        l2.and_then(|node| node.next),
        if current_val >= 10 { 1 } else { 0 },
    );

    Some(current_node)
}

// fn to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
//     let mut dummy_head = Box::new(ListNode::new(0));
//     let mut curr = &mut dummy_head;
//     for val in arr {
//         curr.next = Some(Box::new(ListNode::new(val)));
//         curr = curr.next.as_mut().unwrap();
//     }
//     dummy_head.next
// }

fn to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut curr = &mut dummy_head;

    for val in arr {
        let new_node = Box::new(ListNode::new(val));
        curr.next = Some(new_node);
        if let Some(ref mut next_node) = curr.next {
            curr = next_node;
        }
    }

    dummy_head.next
}

fn print_linked_list(list: Option<Box<ListNode>>) {
    let mut curr = list;
    while let Some(node) = curr {
        print!("{} ", node.val);
        curr = node.next;
    }
    println!();
}

struct Solution;
fn main() {
    let l1 = to_linked_list(vec![2, 4, 3]); // Represents 342
    let l2 = to_linked_list(vec![5, 6, 4]); // Represents 465
    let l3 = to_linked_list(vec![1, 4, 2]); // Represents 342
    let l4 = to_linked_list(vec![4, 6, 5]); // Represents 465

    let result = Solution::add_two_numbers(l1, l2); // Should represent 807 (7 -> 0 -> 8)
    let result2 = Solution::add_two_numbers(l3, l4); // Should represent 807 (7 -> 0 -> 8)

    print_linked_list(result); // Expected output: 7 0 8
    print_linked_list(result2); // Expected output: 7 0 8
}
