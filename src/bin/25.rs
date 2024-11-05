// Define ListNode struct
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

// Solution struct with reverse_k_group function
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut node = &mut head;
        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return head;
            }
        }

        let mut prev = Self::reverse_k_group(node.take(), k);
        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

// Helper function to create a linked list from a vector
fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = current;
        current = Some(node);
    }
    current
}

// Helper function to print a linked list
fn print_linked_list(mut head: Option<Box<ListNode>>) {
    while let Some(node) = head {
        print!("{} -> ", node.val);
        head = node.next;
    }
    println!("None");
}

struct Solution;
// Example usage
fn main() {
    // Create a linked list from a vector
    let list = to_linked_list(vec![1, 2, 3, 4, 5]);

    // Set k = 2 and reverse every 2 nodes in the list
    let result = Solution::reverse_k_group(list, 2);

    // Print the result
    print_linked_list(result);
}
