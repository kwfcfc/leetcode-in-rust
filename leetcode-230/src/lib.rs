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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut counter = k;
        let mut result = -1;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, counter: &mut i32, result: &mut i32) -> () {
            if root.is_none() {
                return;
            }
            let root_node = root.unwrap();
            let root_ref = root_node.borrow();
            dfs(root_ref.left.clone(), counter, result);
            if *counter == 0 {
                return;
            }
            *counter -= 1;
            if *counter == 0 {
                *result = root_ref.val;
            }
            dfs(root_ref.right.clone(), counter, result);
        }
        dfs(root, &mut counter, &mut result);
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
