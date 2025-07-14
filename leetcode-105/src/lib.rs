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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() {
                return None;
            }
            let mut root = TreeNode::new(preorder[0]);
            let root_inorder = inorder.iter().position(|&r| r == preorder[0]).unwrap_or(0);

            let left_preorder = if root_inorder != 0 { &preorder[1..=root_inorder] } else { &[] };
            let left_inorder = if !left_preorder.is_empty() { &inorder[0..root_inorder] } else { left_preorder };
            let right_preorder = if root_inorder != preorder.len() - 1 { &preorder[root_inorder+1..] } else { &[] };
            let right_inorder = if !right_preorder.is_empty() { &inorder[root_inorder+1..] } else { right_preorder };

            root.left = tree_helper(left_preorder, left_inorder);
            root.right = tree_helper(right_preorder, right_inorder);
            return Some(Rc::new(RefCell::new(root)));
        }
        tree_helper(&preorder, &inorder)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, vec};

    use super::*;

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
    macro_rules! tree_traversal_test {
        ($name: ident, ($preorder: expr, $inorder: expr) => $output: expr) => {
            #[test]
            fn $name() {
                let tree = Solution::build_tree($preorder, $inorder);
                let result = to_vec(tree);
                assert_eq!(result, $output);
            }
        };
    }

    tree_traversal_test!(test_simple, (vec![3,9,20,15,7], vec![9,3,15,20,7]) => vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)]);
    tree_traversal_test!(test_single, (vec![-1], vec![-1]) => vec![Some(-1)]);
    tree_traversal_test!(test_no_right, (vec![-1,2], vec![2,-1]) => vec![Some(-1),Some(2)]);
    tree_traversal_test!(test_no_left, (vec![-1,2], vec![-1,2]) => vec![Some(-1),None,Some(2)]);
}