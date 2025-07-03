// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(
        root_node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root_node.clone()?;
        let mut borrow = root.borrow_mut();
        let temp = Self::invert_tree(borrow.left.clone());
        borrow.left = Self::invert_tree(borrow.right.clone());
        borrow.right = temp;
        root_node
    }
}
#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, vec};

    use super::*;

    fn from_vec(data: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(
            data: &[Option<i32>],
            i: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if i >= data.len() {
                return None;
            }
            match data[i] {
                Some(value) => {
                    let node = Rc::new(RefCell::new(TreeNode {
                        val: value,
                        left: helper(data, 2 * i + 1),
                        right: helper(data, 2 * i + 2),
                    }));
                    Some(node)
                }
                None => None,
            }
        }
        helper(data, 0)
    }

    fn to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            match node {
                Some(rc_node) => {
                    let node_ref = rc_node.borrow();
                    res.push(Some(node_ref.val));
                    queue.push_back(node_ref.left.clone());
                    queue.push_back(node_ref.right.clone());
                }
                None => {
                    res.push(None);
                }
            }
        }

        // filter out None
        while res.last() == Some(&None) {
            res.pop();
        }

        res
    }

    #[macro_export]
    macro_rules! tree_test {
        ($name: ident, $input: expr => $output: expr) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree_vec: Vec<Option<i32>> =
                    original.into_iter().map(Some).collect();
                let tree = from_vec(&tree_vec);

                let inverted = Solution::invert_tree(tree);

                let inverted_vec = to_vec(inverted);
                let result: Vec<i32> =
                    inverted_vec.into_iter().filter_map(|x| x).collect();
                assert_eq!(result, $output);
            }
        };
    }

    tree_test!(test_large_tree, vec![4,2,7,1,3,6,9] => vec![4,7,2,9,6,3,1]);
    tree_test!(test_small_tree, vec![2,1,3] => vec![2,3,1]);
    tree_test!(test_empty, vec![] => vec![]);
}
