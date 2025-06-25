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

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut vec_queue = VecDeque::new();
        let mut result = Vec::<Vec<i32>>::new();

        let root_node = match root {
            Some(node)  => node,
            None => return result,
        };

        vec_queue.push_back(root_node);

        while !vec_queue.is_empty() {
            let mut level_vec = Vec::<i32>::new();
            let level_length = vec_queue.len();

            for _ in 0..level_length {
                if let Some(current_node) = vec_queue.pop_front() {
                    let borrow = current_node.borrow();

                    level_vec.push(borrow.val);
                    if let Some(left_node) = &borrow.left {
                        vec_queue.push_back(left_node.clone());
                    }

                    if let Some(right_node) = &borrow.right {
                        vec_queue.push_back(right_node.clone());
                    }
                }
            }
            result.push(level_vec);
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
