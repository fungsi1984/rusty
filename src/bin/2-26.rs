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

        while p.is_some() || q.is_some() {
            // let x = p.as_ref().map_or(0, |node| node.val);
            let x = if let Some(node) = &p { node.val } else { 0 };

            let y = q.as_ref().map_or(0, |node| node.val);
            let sum = x + y + carry;
            carry = sum / 10;

            // Create the new node and set `current.next`
            current.next = Some(Box::new(ListNode::new(sum % 10)));

            // Move `current` to `current.next` safely
            if let Some(ref mut next_node) = current.next {
                current = next_node;
            }

            // Update `p` and `q` to their respective next nodes
            p = p.and_then(|node| node.next);
            q = q.and_then(|node| node.next);
        }

        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
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
