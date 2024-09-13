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
    // Helper function to get the value from an Option<Box<ListNode>>
    fn get_val(node: &Option<Box<ListNode>>) -> i32 {
        node.as_ref().map_or(0, |n| n.val)
    }

    // Helper function to add a new node to the linked list
    fn add_node(current: &mut Box<ListNode>, value: i32) {
        current.next = Some(Box::new(ListNode::new(value)));
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = l1;
        let mut p2 = l2;
        let mut carry = 0;

        // Dummy node to help build the resulting linked list
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;

        while p1.is_some() || p2.is_some() || carry != 0 {
            // Use helper function to get values from the current nodes
            let x = Solution::get_val(&p1);
            let y = Solution::get_val(&p2);

            // Calculate the sum and the new carry
            let sum = x + y + carry;
            carry = sum / 10;

            // Add a new node with the current digit
            Solution::add_node(current, sum % 10);

            // Move current to the next node safely without unwrap
            if let Some(ref mut next_node) = current.next {
                current = next_node;
            }

            // Move to the next nodes in l1 and l2
            if let Some(node) = p1 {
                p1 = node.next;
            }
            if let Some(node) = p2 {
                p2 = node.next;
            }
        }

        // Return the next of the dummy head which is the actual result
        dummy_head.next
    }
}

fn to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut curr = &mut dummy_head;

    for val in arr {
        let new_node = Box::new(ListNode::new(val));
        curr.next = Some(new_node);
        if let Some(ref mut next_node) = curr.next {
            curr = next_node;
        }
    }

    dummy_head.next
}

fn print_linked_list(list: Option<Box<ListNode>>) {
    let mut curr = list;
    while let Some(node) = curr {
        print!("{} ", node.val);
        curr = node.next;
    }
    println!();
}

struct Solution;
fn main() {
    let l1 = to_linked_list(vec![2, 4, 3]); // Represents 342
    let l2 = to_linked_list(vec![5, 6, 4]); // Represents 465
    let l3 = to_linked_list(vec![1, 4, 2]); // Represents 342
    let l4 = to_linked_list(vec![4, 6, 5]); // Represents 465

    let result = Solution::add_two_numbers(l1, l2); // Should represent 807 (7 -> 0 -> 8)
    let result2 = Solution::add_two_numbers(l3, l4); // Should represent 807 (7 -> 0 -> 8)

    print_linked_list(result); // Expected output: 7 0 8
    print_linked_list(result2); // Expected output: 7 0 8
}
