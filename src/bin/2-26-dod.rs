// try data oriented

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

struct DigitWithCarry {
    digit: i32,
    carry: i32,
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Convert linked lists to arrays of DigitWithCarry structs
        let mut num1 = Self::linked_list_to_array(l1);
        let mut num2 = Self::linked_list_to_array(l2);

        // Ensure num1 is the longer array
        if num1.len() < num2.len() {
            std::mem::swap(&mut num1, &mut num2);
        }

        // Perform addition
        let result = Self::add_arrays(num1, num2);

        // Convert the result array back to a linked list
        Self::array_to_linked_list(result)
    }

    // Convert a linked list to an array of DigitWithCarry structs
    fn linked_list_to_array(mut head: Option<Box<ListNode>>) -> Vec<DigitWithCarry> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(DigitWithCarry { digit: node.val, carry: 0 });
            head = node.next;
        }
        result
    }

    // Convert an array of DigitWithCarry structs back to a linked list
    fn array_to_linked_list(arr: Vec<DigitWithCarry>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        for dc in arr {
            current.next = Some(Box::new(ListNode::new(dc.digit)));
            current = current.next.as_mut().unwrap();
        }
        dummy.next
    }

    // Add two arrays of DigitWithCarry structs representing numbers
    fn add_arrays(num1: Vec<DigitWithCarry>, num2: Vec<DigitWithCarry>) -> Vec<DigitWithCarry> {
        let mut result = Vec::new();
        let mut carry = 0;
        let mut i = 0;

        // Process the digits of both numbers
        while i < num2.len() {
            let sum = num1[i].digit + num2[i].digit + carry;
            result.push(DigitWithCarry { digit: sum % 10, carry: sum / 10 });
            carry = sum / 10;
            i += 1;
        }

        // Process the remaining digits of the longer number
        while i < num1.len() {
            let sum = num1[i].digit + carry;
            result.push(DigitWithCarry { digit: sum % 10, carry: sum / 10 });
            carry = sum / 10;
            i += 1;
        }

        // Handle any remaining carry
        if carry > 0 {
            result.push(DigitWithCarry { digit: carry, carry: 0 });
        }

        result
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
