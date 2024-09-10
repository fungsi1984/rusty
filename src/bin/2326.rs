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
        // let n = n as usize;
        // let m = m as usize;
        // let mut spiral_matrix = vec![vec![-1; n]; m];
        let mut spiral_matrix = vec![vec![-1; n as usize]; m as usize];

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
    // Create a linked list: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
    let mut node9 = Box::new(ListNode::new(9));
    let mut node8 = Box::new(ListNode::new(8));
    let mut node7 = Box::new(ListNode::new(7));
    let mut node6 = Box::new(ListNode::new(6));
    let mut node5 = Box::new(ListNode::new(5));
    let mut node4 = Box::new(ListNode::new(4));
    let mut node3 = Box::new(ListNode::new(3));
    let mut node2 = Box::new(ListNode::new(2));
    let mut node1 = Box::new(ListNode::new(1));

    // Connect the nodes
    node8.next = Some(node9);
    node7.next = Some(node8);
    node6.next = Some(node7);
    node5.next = Some(node6);
    node4.next = Some(node5);
    node3.next = Some(node4);
    node2.next = Some(node3);
    node1.next = Some(node2);

    // Call the spiral_matrix function with the linked list
    let result = Solution::spiral_matrix(3, 3, Some(node1));

    // Print the result matrix
    for row in result {
        println!("{:?}", row);
    }
}
