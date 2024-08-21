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

type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(mut l1: Node, mut l2: Node) -> Node {
        let mut carry = 0;
        let mut head = Box::new(ListNode::new(0));
        let mut cur = &mut head;

        loop {
            // Check the current state of l1, l2, and carry
            match (&l1, &l2, carry) {
                (None, None, 0) => break, // Exit condition when all are exhausted
                _ => {
                    let mut sum = carry;

                    // Extract value from l1 if it exists, then update l1
                    if let Some(node) = l1 {
                        sum += node.val;
                        l1 = node.next;
                    }

                    // Extract value from l2 if it exists, then update l2
                    if let Some(node) = l2 {
                        sum += node.val;
                        l2 = node.next;
                    }

                    carry = sum / 10;
                    cur.next = Some(Box::new(ListNode::new(sum % 10)));

                    if let Some(ref mut next_node) = cur.next {
                        cur = next_node;
                    }
                }
            }
        }

        head.next
    }
}

fn array_to_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut cur = &mut head;

    for &val in arr.iter() {
        let node = Box::new(ListNode::new(val));
        *cur = Some(node);
        if let Some(ref mut inner_node) = cur {
            cur = &mut inner_node.next;
        }
    }

    head
}

fn list_to_array(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut arr = Vec::new();
    let mut cur = list;

    while let Some(node) = cur {
        arr.push(node.val);
        cur = node.next;
    }

    arr
}

fn print_array(arr: Vec<i32>) {
    print!("[");
    for (i, &val) in arr.iter().enumerate() {
        print!("{}", val);
        if i < arr.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}
struct Solution;
fn main() {
    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];

    let l1 = array_to_list(l1);
    let l2 = array_to_list(l2);

    let result = Solution::add_two_numbers(l1, l2);
    let result = list_to_array(result);
    print_array(result); // Output: [7, 0, 8]
}
