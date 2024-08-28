// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn add_two_numbers_addition(
        addition: i32,
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1box), Some(l2box)) => {
                let ListNode {
                    val: val1,
                    next: next1,
                } = *l1box;
                let ListNode {
                    val: val2,
                    next: next2,
                } = *l2box;
                let mut val = val1 + val2 + addition;
                let mut current_addition = 0;
                if val >= 10 {
                    val = val - 10;
                    current_addition = 1;
                }
                let mut renode = ListNode::new(val);
                renode.next = Self::add_two_numbers_addition(current_addition, next1, next2);
                Some(Box::new(renode))
            }
            (Some(l1box), None) => {
                let ListNode {
                    val: val1,
                    next: next1,
                } = *l1box;
                let mut val = val1 + addition;
                let mut current_addition = 0;
                if val >= 10 {
                    val = val - 10;
                    current_addition = 1;
                }
                let mut renode = ListNode::new(val);
                renode.next = Self::add_two_numbers_addition(current_addition, next1, None);
                Some(Box::new(renode))
            }
            (None, Some(l2box)) => {
                let ListNode {
                    val: val2,
                    next: next2,
                } = *l2box;
                let mut val = val2 + addition;
                let mut current_addition = 0;
                if val >= 10 {
                    val = val - 10;
                    current_addition = 1;
                }
                let mut renode = ListNode::new(val);
                renode.next = Self::add_two_numbers_addition(current_addition, None, next2);

                Some(Box::new(renode))
            }
            (None, None) => {
                if addition != 0 {
                    return Some(Box::new(ListNode::new(1)));
                }
                None
            }
        }
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_addition(0, l1, l2)
    }
}

// fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
//     if arr.is_empty() {
//         return None;
//     }

//     let mut head = Some(Box::new(ListNode::new(arr[0])));
//     let mut current = head.as_mut().unwrap();

//     for &val in arr.iter().skip(1) {
//         current.next = Some(Box::new(ListNode::new(val)));
//         current = current.next.as_mut().unwrap();
//     }

//     head
// }
//
//

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

struct Solution;

// fn main() {
//     let l1 = array_to_linked_list(vec![2, 4, 3]);
//     let l2 = array_to_linked_list(vec![5, 6, 4]);

//     let result = Solution::add_two_numbers(l1, l2);

//     // Print the result
//     let mut current = result;
//     while let Some(node) = current {
//         print!("{} -> ", node.val);
//         current = node.next;
//     }
//     println!("None");
// }

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
