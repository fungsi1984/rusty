#[derive(PartialEq, Eq, Clone, Debug)]
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
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        let mut head = Box::new(ListNode::new(0));
        let mut cur = &mut head;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;

            // Create the next node
            let next_node = Some(Box::new(ListNode::new(sum % 10)));

            // Instead of mutating cur directly, store the next node
            cur.next = next_node;

            // Move `cur` forward by temporarily assigning a reference to cur.next
            cur = match cur.next.as_mut() {
                Some(next) => next,
                None => break, // This should never happen in this logic, but just in case
            };
        }

        head.next
    }
}

struct Solution;

fn array_to_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in arr.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }
    head
}

fn list_to_array(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut arr = Vec::new();
    while let Some(node) = list {
        arr.push(node.val);
        list = node.next;
    }
    arr
}

fn main() {
    let l1 = array_to_list(vec![2, 4, 3]);
    let l2 = array_to_list(vec![5, 6, 4]);

    let result = Solution::add_two_numbers(l1, l2);
    let result = list_to_array(result);

    println!("{:?}", result); // Output: [7, 0, 8]
}
