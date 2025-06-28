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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_node) = root {
            let root_ref = root_node.borrow();
            1 + max(
                Self::max_depth(root_ref.left.clone()),
                Self::max_depth(root_ref.right.clone()),
            )
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_vec(data: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(data: &[Option<i32>], i: usize) -> Option<Rc<RefCell<TreeNode>>> {
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
    macro_rules! tree_depth_test {
        ($name: ident, $input: expr => $output: literal) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree = from_vec(&original);
                let result = Solution::max_depth(tree);
                assert_eq!(result, $output);
            }
        };
    }

    tree_depth_test!(test_simple, vec![Some(1),None,Some(2)] => 2);

    tree_depth_test!(test_normal, vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)] => 3);
}
