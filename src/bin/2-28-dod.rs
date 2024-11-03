// Definition for singly-linked list.
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

// Struct to store the state of the addition operation
struct State<'a> {
    l1: &'a mut Option<Box<ListNode>>,
    l2: &'a mut Option<Box<ListNode>>,
    carry: i32,
}

impl<'a> State<'a> {
    // Helper function to process each digit addition step
    fn add_step(&mut self) -> i32 {
        let v1 = if let Some(node) = self.l1.take() {
            *self.l1 = node.next;
            node.val
        } else {
            0
        };

        let v2 = if let Some(node) = self.l2.take() {
            *self.l2 = node.next;
            node.val
        } else {
            0
        };

        let sum = v1 + v2 + self.carry;
        self.carry = sum / 10; // Update carry for the next step

        sum % 10 // Return the current digit
    }

    // Check if any more digits need to be processed
    fn has_more(&self) -> bool {
        self.l1.is_some() || self.l2.is_some() || self.carry > 0
    }
}

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = &mut head;

        let mut state = State {
            l1: &mut l1,
            l2: &mut l2,
            carry: 0,
        };

        while state.has_more() {
            let digit = state.add_step();

            // Insert the calculated digit as a new ListNode
            if let Some(ref mut node) = current {
                node.next = Some(Box::new(ListNode::new(digit)));
                current = &mut node.next;
            }
        }


        // how to handle unwrap

        // head.and_then(|node| node.next)
        
        // head.unwrap().next // Return the resulting list, skipping the dummy head
        
        // Return head.next if head is Some, otherwise return None
        // if let Some(node) = head {
        //     node.next
        // } else {
        //     None
        // }

        // use match
        match head {
            Some(node) => node.next,
            None => None,
        }
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
