use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        use std::collections::HashMap;
        fn dfs_collect_duplicate_subtrees(
            root: Option<Rc<RefCell<TreeNode>>>,
            fr: &mut HashMap<Vec<i32>, u16>,
            tres: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Vec<i32> {
            if let Some(rc_node) = root {
                let TreeNode { val, left, right } = &*rc_node.borrow();
                let tree_identifier = [
                    Vec::from([*val]),
                    dfs_collect_duplicate_subtrees(left.clone(), fr, tres),
                    dfs_collect_duplicate_subtrees(right.clone(), fr, tres),
                ]
                .concat();
                if let Some(freq) = fr.get_mut(&tree_identifier) {
                    if *freq == 1 {
                        //collect only when seeing a duplicate for the first time
                        tres.push(Some(rc_node.clone()));
                        *freq += 1;
                    }
                } else {
                    fr.insert(tree_identifier.clone(), 1);
                }
                tree_identifier
            } else {
                vec![-201] //use to identify a null tree arm, according to constraints this is no value
            }
        }
        let mut duplicate_roots = vec![];
        dfs_collect_duplicate_subtrees(root, &mut HashMap::new(), &mut duplicate_roots);
        duplicate_roots
    }
}

fn vec_to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nodes = vec
        .into_iter()
        .map(|val| val.map(|val| Rc::new(RefCell::new(TreeNode::new(val)))))
        .collect::<Vec<_>>();

    for i in 0..nodes.len() {
        if let Some(node) = &nodes[i] {
            let left_index = 2 * i + 1;
            let right_index = 2 * i + 2;

            if left_index < nodes.len() {
                node.borrow_mut().left = nodes[left_index].clone();
            }

            if right_index < nodes.len() {
                node.borrow_mut().right = nodes[right_index].clone();
            }
        }
    }

    nodes[0].clone()
}

struct Solution;
// fn main() {
//     let vec = vec![
//         Some(1),
//         Some(2),
//         Some(3),
//         Some(4),
//         None,
//         Some(2),
//         Some(4),
//         None,
//         None,
//         Some(4),
//     ];
//     let root = vec_to_tree(vec);

//     let result = Solution::find_duplicate_subtrees(root);
//     println!("{:?}", result);
// }

fn main() {
    let vec = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        Some(2),
        Some(4),
        None,
        None,
        Some(4),
    ];
    let root = vec_to_tree(vec);

    let result = Solution::find_duplicate_subtrees(root);
    let output: Vec<Vec<i32>> = result
        .into_iter()
        .map(|node| {
            let mut stack = vec![node];
            let mut values = vec![];
            while let Some(node) = stack.pop() {
                let node = node.unwrap();
                let node = node.borrow();
                values.push(node.val);
                if let Some(right) = &node.right {
                    stack.push(Some(right.clone()));
                }
                if let Some(left) = &node.left {
                    stack.push(Some(left.clone()));
                }
            }
            values
        })
        .collect();
    println!("{:?}", output);
}
