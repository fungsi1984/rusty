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
        let mut l1_digit = l1;
        let mut l2_digit = l2;
        let mut carry = 0;
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        while l1_digit.is_some() || l2_digit.is_some() {
            let mut sum = carry;

            if let Some(node) = l1_digit {
                sum += node.val;
                l1_digit = node.next;
            }

            if let Some(node) = l2_digit {
                sum += node.val;
                l2_digit = node.next;
            }

            if sum > 9 {
                carry = 1;
                sum -= 10;
            } else {
                carry = 0;
            }

            if let Some(node) = tail {
                tail = &mut node.next;
            }
            *tail = Some(Box::new(ListNode::new(sum)));
        }

        if carry > 0 {
            if let Some(node) = tail {
                tail = &mut node.next;
            }
            *tail = Some(Box::new(ListNode::new(carry)));
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
