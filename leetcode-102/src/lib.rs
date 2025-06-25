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

use std::cmp::max;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;


impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(node) = root {
            let root_node = node.borrow();
            let mut result: Vec<Vec<i32>> = vec![vec![root_node.val]];

            let left_level = Self::level_order(root_node.left.clone());
            let right_level = Self::level_order(root_node.right.clone());
            result.extend(Self::merge_vec(left_level, right_level));
            return result;
        }
        vec![]
    }

    fn merge_vec(left: Vec<Vec<i32>>, right: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let result_len = max(left.len(), right.len());
        let mut result = Vec::<Vec<i32>>::with_capacity(result_len);
        for i in 0..result_len {
            match (left.get(i), right.get(i)) {
                (Some(v_left), Some(v_right)) => {
                    let mut merged = v_left.clone();
                    merged.extend_from_slice(&v_right);
                    result.push(merged);
                }
                (Some(v_left), None) => result.push(v_left.clone()),
                (None, Some(v_right)) => result.push(v_right.clone()),
                (None, None) => {}
            }
        }
        result
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
    macro_rules! tree_traversal_test {
        ($name: ident, $input: expr => $output: expr) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree = from_vec(&original);
                let result = Solution::level_order(tree);
                assert_eq!(result, $output);
            }
        };
    }

    tree_traversal_test!(test_normal, vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)] => vec![vec![3],vec![9,20],vec![15,7]]);
    tree_traversal_test!(test_single, vec![Some(1)] => vec![vec![1]]);
    tree_traversal_test!(test_empty, Vec::<Option<i32>>::new() => Vec::<Vec<i32>>::new());
}
