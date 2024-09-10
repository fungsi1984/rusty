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
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![-1; n as usize]; m as usize];
        let mut row_st = 0;
        let mut row_end = m - 1;
        let mut col_st = 0;
        let mut col_end = n - 1;
        let mut head = head;
        match head {
            Some(node) => {
                res[0][0] = node.val;
                head = node.next;
            }
            None => return res,
        };
        let mut r = 0;
        let mut c = 0;
        while let Some((rr, cc)) =
            Self::next_pos(r, c, &mut row_st, &mut row_end, &mut col_st, &mut col_end)
        {
            match head {
                Some(node) => {
                    res[rr as usize][cc as usize] = node.val;
                    // iter
                    r = rr;
                    c = cc;
                    head = node.next;
                }
                None => break,
            }
        }
        res
    }
    fn next_pos(
        r: i32,
        c: i32,
        row_st: &mut i32,
        row_end: &mut i32,
        col_st: &mut i32,
        col_end: &mut i32,
    ) -> Option<(i32, i32)> {
        if c < *col_end && r == *row_st {
            // top
            Some((r, c + 1))
        } else if c == *col_end && r < *row_end {
            // right
            Some((r + 1, c))
        } else if r == *row_end && c > *col_st {
            // down
            Some((r, c - 1))
        } else if c == *col_st && r > *row_st + 1 {
            // left
            Some((r - 1, c))
        } else if r == *row_st + 1 && c == *col_st {
            // update edge
            *row_st += 1;
            *row_end -= 1;
            *col_st += 1;
            *col_end -= 1;
            Some((r, c + 1))
        } else {
            None
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
