// #[derive(PartialEq, Eq, Clone, Debug)]
#[derive(Clone)]
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

// Iterator for ListNode with borrowing and cloning
pub struct ListNodeIterator {
    cur_iter: Option<Box<ListNode>>,
}

impl ListNodeIterator {
    fn new(head: Option<Box<ListNode>>) -> Self {
        ListNodeIterator { cur_iter: head }
    }
}

// Implement Iterator for ListNodeIterator
impl Iterator for ListNodeIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // If cur_iter is None, return None
        match &self.cur_iter {
            Some(node) => {
                // Clone the current node pointer and move to the next
                let val = node.val;
                self.cur_iter = node.next.clone();
                Some(val)
            }
            None => None, // No more items
        }
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
            // cur = cur.next.as_mut().unwrap();
            // Safely move `cur` to cur.next
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
        }

        head.next
    }
}

fn array_to_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut cur = &mut head;

    for &val in arr.iter() {
        let node = Box::new(ListNode::new(val));
        *cur = Some(node);
        // cur = &mut cur.as_mut().unwrap().next;
        if let Some(ref mut next) = cur {
            cur = &mut next.next;
        }
    }

    head
}

fn list_to_array(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut iter = ListNodeIterator::new(list);
    let mut arr = Vec::new();

    while let Some(val) = iter.next() {
        arr.push(val);
    }

    arr
}

fn print_array(arr: Vec<i32>) {
    print!("[");
    for (i, &val) in arr.iter().enumerate() {
        print!("{}", val);
        if i < arr.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}
struct Solution;
fn main() {
    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];

    let l1 = array_to_list(l1);
    let l2 = array_to_list(l2);

    let result = Solution::add_two_numbers(l1, l2);
    let result = list_to_array(result);
    print_array(result); // Output: [7, 0, 8]
}
