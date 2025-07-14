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
        fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i16 {
            if let Some(tree) = node {
                let borrow = tree.borrow();
                let left_depth = depth(&borrow.left);
                let right_depth = depth(&borrow.right);
                if left_depth == -1 || right_depth == -1 {
                    -1
                } else if (left_depth - right_depth).abs() <= 1 {
                    1 + max(left_depth, right_depth)
                } else {
                    -1
                }
            }
            else {
                0
            }
        }
        depth(&root) != -1
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
