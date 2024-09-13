use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;

// TreeNode definition
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(x: i32) -> Self {
        TreeNode {
            val: x,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn recover_from_preorder(
        traversal: String,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        make_tree(&mut NodeIter::new(&traversal).peekable())
    }
}

fn make_tree(nodes: &mut Peekable<NodeIter>) -> Option<Rc<RefCell<TreeNode>>> {
    let data = nodes.next()?;
    let mut node = TreeNode::new(data.val);
    let depth = data.depth;
    if let Some(next) = nodes.peek() {
        if next.depth == depth + 1 {
            node.left = make_tree(nodes);
        }
    }
    if let Some(next) = nodes.peek() {
        if next.depth == depth + 1 {
            node.right = make_tree(nodes);
        }
    }
    Some(Rc::new(RefCell::new(node)))
}

struct NodeIter<'a> {
    parser: Parser<'a>,
}

impl<'a> NodeIter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            parser: Parser::new(s),
        }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        let count = self.parser.parse_n(b'-');
        let num = self.parser.parse_number()?;
        Some(Node {
            val: num,
            depth: count,
        })
    }
}

#[derive(Debug)]
struct Node {
    val: i32,
    depth: usize,
}

struct Parser<'a> {
    data: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { data: s.as_bytes() }
    }

    pub fn parse_n(&mut self, sym: u8) -> usize {
        let count = self.data.iter().take_while(|&&c| c == sym).count();
        self.data = &self.data[count..];
        count
    }

    pub fn parse_number<T: std::str::FromStr>(&mut self) -> Option<T> {
        let count = self
            .data
            .iter()
            .take_while(|&&c| c.is_ascii_digit())
            .count();
        let (left, right) = self.data.split_at(count);
        let num = unsafe { std::str::from_utf8_unchecked(left) }
            .parse::<T>()
            .ok()?;
        self.data = right;
        Some(num)
    }
}

fn preorder_traversal(
    root: Option<Rc<RefCell<TreeNode>>>,
    result: &mut Vec<i32>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        // Visit the root node
        result.push(node.val);

        // Visit the left subtree
        preorder_traversal(node.left.clone(), result);

        // Visit the right subtree
        preorder_traversal(node.right.clone(), result);
    }
}

struct Solution;
fn main() {
    // The preorder traversal string where '-' represents the depth
    let traversal = "1-2--3--4-5--6--7".to_string();

    // Recover the binary tree from the traversal string
    let root = Solution::recover_from_preorder(traversal.clone());

    // Vector to store the preorder traversal result
    let mut result = Vec::new();

    // Perform preorder traversal to collect the node values
    preorder_traversal(root, &mut result);

    // Output the result in the desired format
    println!("Input: traversal = \"{}\"", traversal);
    print!("Output: [");
    for (i, &val) in result.iter().enumerate() {
        print!("{}", val);
        if i < result.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}
