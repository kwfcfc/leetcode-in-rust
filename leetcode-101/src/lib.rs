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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn node_eq(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(_), None) => false,
                (None, Some(_)) => false,
                (Some(left_ref), Some(right_ref)) => {
                    let left_node = left_ref.as_ref().borrow();
                    let right_node = right_ref.as_ref().borrow();
                    left_node.val == right_node.val &&
                    node_eq(&left_node.left, &right_node.right) && node_eq(&left_node.right, &right_node.left)
                }
            }
        }
        if let Some(root_node) = root {
            let borrow = root_node.borrow();
            node_eq(&borrow.left, &borrow.right)
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
    macro_rules! tree_symmetric_test {
        ($name: ident, $input: expr => $output: literal) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree = from_vec(&original);
                let result = Solution::is_symmetric(tree);
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

    tree_symmetric_test!(test_true, option_vec![1,2,2,3,4,4,3] => true);
    tree_symmetric_test!(test_false, option_vec![1,2,2,null,3,null,3] => false);
}
