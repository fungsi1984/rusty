struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn add_two_numbers_with_carry(
    l1: &mut Option<Box<ListNode>>,
    l2: &mut Option<Box<ListNode>>,
    carry: bool,
) {
    if let (None, None, false) = (&l1, &l2, carry) {
        return;
    }
    if let (None, None, true) = (&l1, &l2, carry) {
        *l1 = Some(Box::new(ListNode { val: 1, next: None }));
        return;
    }
    if l1.is_none() {
        std::mem::swap(l1, l2);
    }
    if let Some(node) = l1.as_mut() {
        let ListNode {
            val: val1,
            next: next1,
        } = &mut **node;
        let ListNode {
            val: val2,
            next: next2,
        } = match l2.as_mut() {
            Some(v) => v,
            None => &mut ListNode::new(0),
        };
        let new_val = *val1 + *val2 + carry as i32;
        let carry = new_val >= 10;
        let new_val = new_val % 10;
        add_two_numbers_with_carry(next1, next2, carry);
        *val1 = new_val;
    }
}
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        add_two_numbers_with_carry(&mut l1, &mut l2, false);
        l1
    }
}

fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }
    head
}

fn print_linked_list(list: Option<Box<ListNode>>) {
    let mut current = list;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
    println!();
}

struct Solution;
fn main() {
    // Example Test Case
    let l1 = to_linked_list(vec![2, 4, 3]); // Represents 342
    let l2 = to_linked_list(vec![5, 6, 4]); // Represents 465
    let l3 = to_linked_list(vec![1, 4, 2]); // Represents 342
    let l4 = to_linked_list(vec![4, 6, 5]); // Represents 465
    let result = Solution::add_two_numbers(l1, l2); // Should represent 807 (7 -> 0 -> 8)
    let result2 = Solution::add_two_numbers(l3, l4); // Should represent 807 (7 -> 0 -> 8)
    print_linked_list(result); // Expected output: 7 0 8
    print_linked_list(result2); // Expected output: 7 0 8
}
