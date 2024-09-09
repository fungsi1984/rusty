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

impl Solution {
    // The code in this function is a solution to Spiral Matrix II if
    // we use 0 as the unassigned value instead of -1, and assign k+1 instead of k.
    // You can use your own solution code here, as long as that code iterates
    // over the values that are to be assigned into the spiral.
    pub fn spiral_matrix(m: i32, n: i32, node: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        // let mut m = m as usize;
        // let mut n = n as usize;
        let mut spiral_matrix = vec![vec![-1; n as usize ]; m as usize];

        let (mut i, mut j, mut di, mut dj) = (0_i32, 0_i32, 0_i32, 1_i32);
        for k in Sll::new(node) {
            // Iterate over the linked list
            spiral_matrix[i as usize][j as usize] = k;
            if spiral_matrix[((i + di).rem_euclid(m)) as usize][((j + dj).rem_euclid(n)) as usize]
                != -1
            {
                let temp = di;
                di = dj;
                dj = -temp;
            }
            i += di;
            j += dj;
        }
        spiral_matrix
    }
}

// Since you can not impl traits that you did not define yourself
// for types you did not define we (kind of) use a type alias to
// enable us to impl IntoIterator for the linked list.
// This way we can just use a normal for loop over its values.
struct Sll {
    node: Option<Box<ListNode>>,
}

impl Sll {
    const fn new(node: Option<Box<ListNode>>) -> Self {
        Self { node }
    }
}

// All the code below this line just lets us convert
// a singly linked list into an iterator.

impl IntoIterator for Sll {
    type Item = i32;
    type IntoIter = SllWalker;
    fn into_iter(self) -> Self::IntoIter {
        SllWalker::new(self)
    }
}

struct SllWalker {
    node: Option<Box<ListNode>>,
}

impl SllWalker {
    fn new(head: Sll) -> Self {
        Self { node: head.node }
    }
}

impl Iterator for SllWalker {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            Some(ref mut n) => {
                let val = n.val;
                self.node = n.next.take();
                Some(val)
            }
            None => None,
        }
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
