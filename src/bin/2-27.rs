// Definition for singly-linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut current = &mut head;

        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = l1.as_ref().map_or(0, |x| x.val)
                + l2.as_ref().map_or(0, |x| x.val)
                + carry;
            carry = sum / 10;

            let sume_node = current.insert(Box::new(ListNode::new(sum % 10)));
            current = &mut sume_node.next;
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }
        return head;
    }
}

// Helper function to create a linked list from a vector of integers
fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    for val in values {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    dummy.next
}

// Helper function to print a linked list
fn print_list(head: Option<Box<ListNode>>) {
    let mut head = head;
    while let Some(node) = head {
        print!("{} ", node.val);
        head = node.next;
    }
    println!();
}

struct Solution;
fn main() {
    // Example usage
    let l1_values = vec![2, 4, 3]; // Represents the number 342
    let l2_values = vec![5, 6, 4]; // Represents the number 465

    let l1 = create_list(l1_values);
    let l2 = create_list(l2_values);

    let result = Solution::add_two_numbers(l1, l2);

    print!("Result: ");
    print_list(result);
}

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
