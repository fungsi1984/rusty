// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        let mut head = Box::new(ListNode::new(0));
        let mut cur = &mut head;

        while l1.is_some() || l2.is_some() || carry > 0 {
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
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            cur = cur.next.as_mut().unwrap();
        }

        head.next
    }

    fn array_to_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut cur = &mut head;

        for &val in arr.iter().rev() {
            let node = Box::new(ListNode::new(val));
            *cur = Some(node);
            cur = &mut cur.as_mut().unwrap().next;
        }

        head
    }

    fn list_to_array(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut arr = Vec::new();
        let mut cur = list;

        while let Some(node) = cur {
            arr.push(node.val);
            cur = node.next;
        }

        arr.reverse();
        arr
    }

    fn print_list(list: Option<Box<ListNode>>) {
        let mut cur = list;
        while let Some(node) = cur {
            print!("{}", node.val);
            cur = node.next;
        }
        println!();
    }
}

struct Solution;
// fn main() {
//     let mut l1 = Some(Box::new(ListNode::new(2)));
//     l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
//     l1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

//     let mut l2 = Some(Box::new(ListNode::new(5)));
//     l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
//     l2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

//     let mut result = Solution::add_two_numbers(l1, l2);

//     while let Some(node) = result {
//         print!("{}", node.val);
//         result = node.next;
//     }
// }

fn main() {
    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];

    let l1 = Solution::array_to_list(l1);
    let l2 = Solution::array_to_list(l2);

    let result = Solution::add_two_numbers(l1, l2);
    Solution::print_list(result); // Output: 708
}
