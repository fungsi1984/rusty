use std::boxed::Box;
use std::cmp::min;
use std::option::Option;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(ref mut curr_node) = current {
            if let Some(ref mut next_node) = curr_node.next {
                // Calculate GCD of current node value and next node value
                let gcd_val = Self::gcd(curr_node.val, next_node.val);

                // Create a new node with the GCD value
                let new_node = Box::new(ListNode {
                    val: gcd_val,
                    next: curr_node.next.take(),
                });

                // Insert the new node between curr_node and next_node
                curr_node.next = Some(new_node);

                // Move to the next original node
                current = &mut curr_node.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }

        head
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

impl ListNode {
    // Helper function to create a linked list from a vector
    pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &val in values.iter().rev() {
            let new_node = Box::new(ListNode {
                val,
                next: current.take(),
            });
            *current = Some(new_node);
        }

        head
    }

    // Helper function to print the linked list
    pub fn print(head: &Option<Box<ListNode>>) {
        let mut current = head;

        while let Some(node) = current {
            print!("{} ", node.val);
            current = &node.next;
        }
        println!();
    }
}

fn main() {
    // Create a list: 12 -> 15 -> 21
    let values = vec![18, 6, 10, 3];
    let mut head = ListNode::from_vec(values);

    println!("Original List: ");
    ListNode::print(&head);

    head = Solution::insert_greatest_common_divisors(head);

    println!("List after inserting GCDs: ");
    ListNode::print(&head);
}
