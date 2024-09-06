// Definition for singly-linked list.
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = ListNode { val: 0, next: head };
        let mut mp = vec![false; 100001];

        for n in &nums {
            mp[*n as usize] = true;
        }

        let mut cur = &mut root;

        while let Some(nn) = cur.next.as_mut() {
            if mp[nn.val as usize] {
                cur.next = nn.next.take();
            } else {
                if let Some(ref mut next_node) = cur.next {
                    cur = next_node;
                }
            }
        }

        root.next
    }
}

// Function to convert an array into a linked list (ListNode)
fn array_to_linked_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut current = &mut head;

    for &val in arr {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }

    head
}

// Helper function to print the linked list
fn print_list(head: Option<Box<ListNode>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next;
    }
    println!("None");
}

fn main() {
    // Example input: nums = [1, 2, 3], head = [1, 2, 3, 4, 5]
    let nums = vec![1, 2, 3];
    let head_array = vec![1, 2, 3, 4, 5];

    // Convert array into linked list
    let head = array_to_linked_list(&head_array);

    // Modify the list
    let modified_list = Solution::modified_list(nums, head);

    // Print the modified list
    print_list(modified_list);
}
