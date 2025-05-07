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

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                result.push(node.val);
                dfs(&node.left, result);
                dfs(&node.right, result);
            }
        }

        if root.is_none() {
            return;
        }

        let mut result_vector = Vec::<i32>::new();
        dfs(root, &mut result_vector);

        let mut iter = root.clone();
        for value in &result_vector[1..] {
            if let Some(node) = iter {
                let mut borrowed = node.borrow_mut();

                borrowed.left = None;
                borrowed.right = Some(Rc::new(RefCell::new(TreeNode::new(*value))));
                
                iter = borrowed.right.clone();
            }
        }
    }
}

#[cfg(test)]
mod test {
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

    fn flattened_to_vec(mut node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        while let Some(current) = node {
            let borrowed = current.borrow();
            result.push(borrowed.val);
            node = borrowed.right.clone();
        }
        result
    }

    #[macro_export]
    macro_rules! flatten_test {
        ($name: ident, $input: expr => $output: expr) => {
            #[test]
            fn $name() {
                let original = $input;
                let mut tree = from_vec(&original);

                Solution::flatten(&mut tree);
                assert_eq!(flattened_to_vec(tree), $output);
            }
        };
    }

    flatten_test!(test_empty, vec![] => vec![]);
    flatten_test!(test_single, vec![Some(1)] => vec![1]);
    flatten_test!(
        test_sample,
        vec![Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)] =>
        vec![1, 2, 3, 4, 5, 6]
    );
}
