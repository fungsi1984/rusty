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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>, 
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode::new(0));
        let mut tail = &mut sentinel;
        let (mut l1, mut l2, mut carry) = (l1, l2, 0);

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = carry 
                + l1.as_ref().map_or(0, |n| n.val)
                + l2.as_ref().map_or(0, |n| n.val);

            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            // match tail.next.as_mut() {
            //     Some(t) => tail = t,
            //     None => panic!("Unexpected None while updating tail"),
            // }

            match tail.next.as_mut() {
                Some(t) => tail = t,
                None => break, // If `None`, gracefully exit the loop
            }

            carry = sum / 10;
            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }

        sentinel.next
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