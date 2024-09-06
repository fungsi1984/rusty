struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
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
    // else {
    //     // Handle the case where l1 is None (which should be already swapped or processed)
    //     *l1 = None;
    // }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        add_two_numbers_with_carry(&mut l1, &mut l2, false);
        l1
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
