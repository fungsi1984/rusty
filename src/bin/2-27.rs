// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = l1;
        let mut q = l2;
        let mut carry = 0;
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;

        loop {
            let (x, next_p) = match p {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };

            let (y, next_q) = match q {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };

            let sum = x + y + carry;
            carry = sum / 10;

            current.next = Some(Box::new(ListNode::new(sum % 10)));
            // current = current.next.as_mut().unwrap();
            if let Some(ref mut next_node) = current.next {
                current = next_node;
            }

            p = next_p;
            q = next_q;

            // Break when there are no more nodes and no carry
            if p.is_none() && q.is_none() && carry == 0 {
                break;
            }
            
        }

        dummy_head.next
    }
}

fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in vec.iter().rev() {
        let new_node = Some(Box::new(ListNode::new(val))); // Create the new node
        *current = new_node; // Assign it to the current position

        // Move the current pointer to the next position
        current = if let Some(node) = current {
            &mut node.next
        } else {
            // This case should not happen since we just created a new node
            unreachable!()
        };
    }

    head
}

fn main() {
    // Example usage
    let l1 = from_vec(vec![2, 4, 3]);
    let l2 = from_vec(vec![5, 6, 4]);

    let result = Solution::add_two_numbers(l1, l2);

    // Print the result
    let mut current = result;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next;
    }
    println!("None");
}
