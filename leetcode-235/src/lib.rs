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
use std::cmp::{max, min};
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_ref = root?;
        let (p_ref, q_ref) = (p?, q?);

        let (root_val, p_val, q_val) = (
            root_ref.borrow().val,
            p_ref.borrow().val,
            q_ref.borrow().val,
        );

        let (smaller, larger) = (min(p_val, q_val), max(p_val, q_val));

        if root_val < smaller {
            Self::lowest_common_ancestor(root_ref.borrow().right.clone(), Some(p_ref), Some(q_ref))
        } else if root_val > larger {
            Self::lowest_common_ancestor(root_ref.borrow().left.clone(), Some(p_ref), Some(q_ref))
        } else {
            Some(root_ref)
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

    fn find_two_nodes(
        root: &Option<Rc<RefCell<TreeNode>>>,
        val1: i32,
        val2: i32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        fn find_node(
            node: &Option<Rc<RefCell<TreeNode>>>,
            target: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let mut cur = node.clone();
            while let Some(n) = cur {
                let n_borrow = n.borrow();
                if n_borrow.val == target {
                    return Some(n.clone());
                } else if target < n_borrow.val {
                    cur = n_borrow.left.clone();
                } else {
                    cur = n_borrow.right.clone();
                }
            }
            None
        }

        let node1 = find_node(root, val1);
        let node2 = find_node(root, val2);
        (node1, node2)
    }

    #[macro_export]
    macro_rules! tree_ancestor_test {
        ($name: ident, ($input: expr, $p: literal, $q: literal) => $output: expr) => {
            #[test]
            fn $name() {
                let original = $input;
                let tree = from_vec(&original);
                let (p, q) = find_two_nodes(&tree, $p, $q);
                let result = Solution::lowest_common_ancestor(tree, p, q).unwrap();
                let result_ref = result.borrow();
                assert_eq!(result_ref.val, $output);
            }
        };
    }

    tree_ancestor_test!(test_child, (vec![Some(6),Some(2),Some(8),Some(0),Some(4),Some(7),Some(9),None,None,Some(3),Some(5)], 2, 4) => 2);
    tree_ancestor_test!(test_sibling, (vec![Some(6),Some(2),Some(8),Some(0),Some(4),Some(7),Some(9),None,None,Some(3),Some(5)], 2, 8) => 6);
    tree_ancestor_test!(test_simple, (vec![Some(1),None,Some(2)], 1, 2) => 1);
}
