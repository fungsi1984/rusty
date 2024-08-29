struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => {
                let val = a.val + b.val;
                let rem = val % 10;
                let b_next = if a.val + b.val >= 10 {
                    Self::add_two_numbers(Some(Box::new(ListNode { val: 1, next: None })), b.next)
                } else {
                    b.next
                };
                Some(Box::new(ListNode {
                    val: rem,
                    next: Self::add_two_numbers(a.next, b_next),
                }))
            }
        }
    }
}

fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    if arr.is_empty() {
        return None;
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    for &val in &arr {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut()?;
    }

    dummy.next
}

fn main() {
    let l1 = array_to_linked_list(vec![2, 4, 3]);
    let l2 = array_to_linked_list(vec![5, 6, 4]);

    let result = Solution::add_two_numbers(l1, l2);

    print!("[");
    let mut current = result;
    let mut first = true;
    while let Some(node) = current {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        print!("{}", node.val);
        current = node.next;
    }
    println!("]");
}
