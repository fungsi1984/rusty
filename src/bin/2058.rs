// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut min_distance = i32::MAX;
        let mut first_ma_index = -1;
        let mut prev_ma_index = -1;
        let mut index = 1;

        let mut prev = head.as_ref(); // Point to the index 0.
        let mut curr = head.as_ref().and_then(|node| node.next.as_ref()); // Point to the index 1.

        while let (Some(p), Some(c), Some(n)) =
            (prev, curr, curr.and_then(|node| node.next.as_ref()))
        {
            if (c.val > p.val && c.val > n.val) || (c.val < p.val && c.val < n.val) {
                if first_ma_index == -1 {
                    first_ma_index = index;
                }
                if prev_ma_index != -1 {
                    min_distance = min_distance.min(index - prev_ma_index);
                }
                prev_ma_index = index;
            }
            prev = curr;
            curr = curr.and_then(|node| node.next.as_ref());
            index += 1;
        }

        if min_distance == i32::MAX {
            vec![-1, -1]
        } else {
            vec![min_distance, prev_ma_index - first_ma_index]
        }
    }
}

// Helper function to create a linked list from a vector
fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in values.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn main() {
    let values = vec![1, 3, 2, 2, 3, 2, 2, 2, 7];
    let head = create_linked_list(values);

    let result = Solution::nodes_between_critical_points(head);
    println!("{:?}", result); // Output should be [2, 3]
}
