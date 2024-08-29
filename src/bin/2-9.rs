#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut curr = &mut dummy_head;
        let mut carry = 0;

        let mut l1 = l1;
        let mut l2 = l2;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let x = l1.as_ref().map_or(0, |node| node.val);
            let y = l2.as_ref().map_or(0, |node| node.val);
            let sum = carry + x + y;
            carry = sum / 10;

            curr.next = Some(Box::new(ListNode::new(sum % 10)));

            // Safely moving to the next node using if let
            if let Some(ref mut next_node) = curr.next {
                curr = next_node;
            }

            l1 = if let Some(node) = l1 { node.next } else { None };
            l2 = if let Some(node) = l2 { node.next } else { None };
        }

        dummy_head.next
    }
}

fn array_to_list_node(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(arr[0])));
    let mut curr = &mut head;

    for &val in &arr[1..] {
        let next_node = Some(Box::new(ListNode::new(val)));
        if let Some(ref mut current) = curr {
            current.next = next_node;
            curr = &mut current.next;
        }
    }

    head
}

fn print_list_node(head: Option<Box<ListNode>>) {
    let mut head = head;
    print!("[");
    while let Some(node) = head {
        print!("{}", node.val);
        head = node.next;
        if head.is_some() {
            print!(", ");
        }
    }
    println!("]");
}

fn main() {
    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];

    let list1 = array_to_list_node(&l1);
    let list2 = array_to_list_node(&l2);

    let result = Solution::add_two_numbers(list1, list2);
    print_list_node(result);
}
