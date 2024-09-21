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
    pub fn add_two_numbers(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode::new(i32::MAX));
        Self::add_two_numbers_helper(list1, list2, 0, &mut sentinel);
        sentinel.next
    }

    pub fn add_two_numbers_helper(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
        carry: i32,
        sentinel: &mut Box<ListNode>,
    ) {
        match (list1, list2, carry) {
            (None, None, 0) => return,
            (l1, l2, c) => {
                let (val1, next1) = match l1 {
                    Some(node1) => (node1.val, node1.next),
                    None => (0, None),
                };
                
                let (val2, next2) = match l2 {
                    Some(node2) => (node2.val, node2.next),
                    None => (0, None),
                };

                let sum = val1 + val2 + c;
                let next = sentinel.next.insert(Box::new(ListNode::new(sum % 10)));
                Self::add_two_numbers_helper(next1, next2, sum / 10, next)
            }
        }
    }
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
