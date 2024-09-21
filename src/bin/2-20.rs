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

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        let mut head = Box::new(ListNode::new(0));
        let mut cur: *mut Box<ListNode> = &mut head;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;

            unsafe {
                if let Some(node) = l1 {
                    sum += node.val;
                    l1 = node.next;
                }
                if let Some(node) = l2 {
                    sum += node.val;
                    l2 = node.next;
                }

                carry = sum / 10;

                // Using raw pointer dereference to modify the `cur` pointer
                (*cur).next = Some(Box::new(ListNode::new(sum % 10)));

                if let Some(ref mut next) = (*cur).next {
                    cur = next; // Move the pointer to the next node
                }
            }
        }

        head.next
    }
}

fn array_to_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut cur: *mut Option<Box<ListNode>> = &mut head;

    for &val in arr.iter() {
        let node = Box::new(ListNode::new(val));

        unsafe {
            // Using raw pointer dereference
            *cur = Some(node);

            if let Some(ref mut next) = *cur {
                cur = &mut next.next as *mut _; // Move `cur` to point to the next node
            }
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

struct ListNodeIterator {
    cur_iter: Option<Box<ListNode>>,
}

impl ListNodeIterator {
    fn new(head: Option<Box<ListNode>>) -> Self {
        ListNodeIterator { cur_iter: head }
    }
}

impl Iterator for ListNodeIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.cur_iter {
            Some(node) => {
                let val = node.val;
                self.cur_iter = node.next.clone();
                Some(val)
            }
            None => None,
        }
    }
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
