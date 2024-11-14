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

// impl Solution {
//     pub fn add_two_numbers(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         Self::add_two_carry_over(l1, l2, 0)
//     }

//     fn add_two_carry_over(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//         carry_over: i32,
//     ) -> Option<Box<ListNode>> {
//         let (value, (next1, next2)) = match (l1, l2, carry_over) {
//             (None, None, 0) => return None,
//             (Some(n1), Some(n2), c) => {
//                 (n1.val + n2.val + c, (n1.next, n2.next))
//             }
//             (Some(n1), None, c) => (n1.val + c, (n1.next, None)),
//             (None, Some(n2), c) => (n2.val + c, (None, n2.next)),
//             (None, None, c) => (c, (None, None)),
//         };

//         let carry_over = value / 10;
//         let value = value % 10;

//         Some(Box::new(ListNode {
//             next: Self::add_two_carry_over(next1, next2, carry_over),
//             val: value,
//         }))
//     }
// }

// impl Solution {
//     pub fn add_two_numbers(
//         mut l1: Option<Box<ListNode>>,
//         mut l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         let mut l_final = Some(Box::new(ListNode::new(0)));
//         let mut cr = l_final.as_mut();
//         let mut carry = 0;

//         while l1.is_some() || l2.is_some() || carry != 0 {
//             let mut sum = carry;

//             if let Some(node) = l1 {
//                 sum += node.val;
//                 l1 = node.next;
//             }

//             if let Some(node) = l2 {
//                 sum += node.val;
//                 l2 = node.next;
//             }

//             carry = sum / 10;

//             if let Some(current) = cr {
//                 current.next = Some(Box::new(ListNode::new(sum % 10)));
//                 cr = current.next.as_mut();
//             }
//         }

//         l_final.and_then(|node| node.next)
//     }
// }

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l_final = Some(Box::new(ListNode::new(0)));
        let mut carry = 0;
        let mut cr = &mut l_final;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;

            // Using `ref mut` here to access the mutable inner `ListNode`
            match cr {
                Some(ref mut current) => {
                    current.next = Some(Box::new(ListNode::new(sum % 10)));
                    cr = &mut current.next;
                }
                None => break,
            }
        }

        l_final.and_then(|node| node.next)
    }
}



// Helper functions for testing
fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = list;
    while let Some(node) = current {
        vec.push(node.val);
        current = node.next;
    }
    vec
}

struct Solution;
fn main() {
    let l1 = to_linked_list(vec![2, 4, 3]);
    let l2 = to_linked_list(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", to_vec(result)); // Output should be [7, 0, 8]
}
