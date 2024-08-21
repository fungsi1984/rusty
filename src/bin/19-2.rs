#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        remove_nth_from_end_recr(head, n).0
    }
}

fn remove_nth_from_end_recr(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, usize) {
    match head {
        None => (None, 1),
        Some(mut node) => {
            let (prev, num) = remove_nth_from_end_recr(node.next.take(), n);
            if n == num as i32 {
                (prev, num + 1)
            } else {
                node.next = prev;
                (Some(node), num + 1)
            }
        }
    }
}

fn print_list(mut head: Option<Box<ListNode>>) {
    while let Some(node) = head {
        print!("{} ", node.val);
        head = node.next;
    }
    println!();
}

struct Solution;
fn main() {
    // Creating the linked list: 1 -> 2 -> 3 -> 4 -> 5
    let mut node5 = Some(Box::new(ListNode::new(5)));
    let mut node4 = Some(Box::new(ListNode::new(4)));
    let mut node3 = Some(Box::new(ListNode::new(3)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node1 = Some(Box::new(ListNode::new(1)));

    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    node1.as_mut().unwrap().next = node2;

    // Remove the 2nd node from the end, the list should become: 1 -> 2 -> 3 -> 5
    let head = Solution::remove_nth_from_end(node1, 2);

    // Print the modified list
    print_list(head);
}
