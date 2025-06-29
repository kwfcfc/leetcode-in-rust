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
    fn helper(root: Rc<RefCell<TreeNode>>, target_sum: i32) -> Vec<Vec<i32>> {
        let root_ref = root.borrow();
        let mut result = vec![];

        if root_ref.left.is_none() && root_ref.right.is_none() 
            && root_ref.val == target_sum {
            result.push(vec![root_ref.val]);
        }

        if let Some(left) = root_ref.left.clone() {
            let mut left_path = Self::helper(left, target_sum - root_ref.val);
            left_path.iter_mut().for_each(|v| v.push(root_ref.val));
            result.extend(left_path);
        }

        if let Some(right) = root_ref.right.clone() {
            let mut right_path = Self::helper(right, target_sum - root_ref.val);
            right_path.iter_mut().for_each(|v| v.push(root_ref.val));
            result.extend(right_path);
        }

        result
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if let Some(root_node) = root {
            let mut result = Self::helper(root_node, target_sum);
            result.iter_mut().for_each(|v| v.reverse());

            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn from_unbalanced_vec(data: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() || data[0].is_none() {
            return None;
        }
    
        let root = Rc::new(RefCell::new(TreeNode::new(data[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
    
        while i < data.len() {
            let node = queue.pop_front().unwrap();
            // Left child
            if i < data.len() {
                if let Some(val) = data[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            // Right child
            if i < data.len() {
                if let Some(val) = data[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    #[macro_export]
    macro_rules! tree_path_test {
        ($name: ident, ($input: expr, $sum: literal )=> $output: expr) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree = from_unbalanced_vec(&original);
                let result = Solution::path_sum(tree, $sum);
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

    tree_path_test!(test_no_path1, (vec![Some(1),Some(2),Some(3)], 5) 
        => Vec::<Vec<i32>>::new());
    tree_path_test!(test_no_path2, (vec![Some(1),Some(2)], 0) 
        => Vec::<Vec<i32>>::new());
    tree_path_test!(test_normal, 
        (option_vec![5,4,8,11,null,13,4,7,2,null,null,5,1], 22) 
        => vec![vec![5,4,11,2],vec![5,8,4,5]]);
    tree_path_test!(test_manual, 
        (option_vec![5,8,4,13,4,11,null,null,null,5,1,7,2], 22)
        => vec![vec![5,8,4,5],vec![5,4,11,2]]);
}
