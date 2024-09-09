// Define the ListNode structure for the singly linked list
#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    // Constructor to easily create new nodes
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

type List = Option<Box<ListNode>>;

struct ListIter(List);

impl Iterator for ListIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if let Some(mut node) = self.0.take() {
            self.0 = node.next.take();
            Some(node.val)
        } else {
            None
        }
    }
}

impl Solution {
    fn spiral_matrix(m: i32, n: i32, head: List) -> Vec<Vec<i32>> {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut r = (0..m).map(|_| vec![0; n]).collect();
        let f = |v: &mut Vec<Vec<i32>>, (i, j): (usize, usize), x: i32| unsafe {
            *v.get_unchecked_mut(i).get_unchecked_mut(j) = x;
        };
        let mut ls = ListIter(head).chain(std::iter::repeat(-1));
        let p = m.min(n);
        let pp = p / 2;
        let it = (0..pp).flat_map(|k| {
            let mm = m - k - 1;
            let nn = n - k - 1;
            (k..nn)
                .map(move |i| (k, i))
                .chain((k..mm).map(move |i| (i, nn)))
                .chain((k + 1..=nn).rev().map(move |i| (mm, i)))
                .chain((k + 1..=mm).rev().map(move |i| (i, k)))
        });
        for (idx, x) in it.zip(&mut ls) {
            f(&mut r, idx, x);
        }
        if p % 2 != 0 {
            if p == m {
                let it = (pp..n - pp).map(move |i| (pp, i));
                for (idx, x) in it.zip(ls) {
                    f(&mut r, idx, x);
                }
            } else {
                let it = (pp..m - pp).map(move |i| (i, pp));
                for (idx, x) in it.zip(ls) {
                    f(&mut r, idx, x);
                }
            }
        }
        r
    }
}

struct Solution;

fn main() {
    // Create the linked list for head = [3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]
    let values = vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0];
    let m = 3;
    let n = 5;
    // Create the linked list from the values
    let mut head = None;
    for &val in values.iter().rev() {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = head;
        head = Some(new_node);
    }

    // Call the spiral_matrix function with the updated dimensions
    let result = Solution::spiral_matrix(m, n, head);

    // Print the result matrix
    for row in result {
        println!("{:?}", row);
    }
}
