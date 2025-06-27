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
      right: None
    }
  }
}

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn tree_length(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_node) = root {
            let root_ref = root_node.borrow();
            return 1 + Self::tree_length(root_ref.left.clone()) + Self::tree_length(root_ref.right.clone());
        } else {
            0
        }
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if let Some(root_node) = root {
            let root_node_ref = root_node.borrow();

            let left_tree_length = Self::tree_length(root_node_ref.left.clone());

            if k - 1 == left_tree_length {
                root_node_ref.val
            } else if k - 1 < left_tree_length {
                Self::kth_smallest(root_node_ref.left.clone(), k)
            } else {
                Self::kth_smallest(root_node_ref.right.clone(), k - 1 - left_tree_length)
            }
        } else {
            -1
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
    macro_rules! tree_search_test {
        ($name: ident, ($input: expr, $k: literal) => $output: literal) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree = from_vec(&original);
                let result = Solution::kth_smallest(tree, $k);
                assert_eq!(result, $output);
            }
        };
    }

    tree_search_test!(test_simple, (vec![Some(3),Some(1),Some(4),None,Some(2)], 1) => 1);
    tree_search_test!(test_normal, (vec![Some(5),Some(3),Some(6),Some(2),Some(4),None,None,Some(1)], 1) => 1);
}
