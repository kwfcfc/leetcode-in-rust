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
use std::cmp::max;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> u16 {
            if let Some(tree) = node {
                let borrow = tree.borrow();
                1 + max(depth(&borrow.left), depth(&borrow.right))
            } else {
                0
            }
        }
        if let Some(node) = root {
            let borrow = node.borrow();
            let left_depth = depth(&borrow.left);
            let right_depth = depth(&borrow.right);
            let difference = if left_depth > right_depth {
                left_depth - right_depth
            } else {
                right_depth - left_depth
            };

            difference <= 1
                && Self::is_balanced(borrow.left.clone())
                && Self::is_balanced(borrow.right.clone())
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
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

    #[macro_export]
    macro_rules! tree_balance_test {
        ($name: ident, $input: expr => $output: literal) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree = from_vec(&original);
                let result = Solution::is_balanced(tree);
                assert_eq!(result, $output);
            }
        };
    }

    #[macro_export]
    macro_rules! option_vec {
        ( $( null ),* ) => { vec![ $( None ),* ] };
        ( $( $item:tt ),* ) => {
            vec![$( option_vec!(@wrap $item) ),*]
        };
        (@wrap null) => { None };
        (@wrap $val:tt) => { Some($val) };
    }

    tree_balance_test!(test_simple, option_vec![3,9,20,null,null,15,7] => true);
    tree_balance_test!(test_false, option_vec![1,2,2,3,3,null,null,4,4] => false);
    tree_balance_test!(test_empty, Vec::<Option<i32>>::new() => true);
}
