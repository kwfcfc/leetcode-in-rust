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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_node = root?;
        let p_node = p?;
        let q_node = q?;

        if p_node == root_node {
            return Some(p_node);
        } else if q_node == root_node {
            return Some(q_node);
        }

        let root_ref = root_node.borrow();

        let (left_node, right_node) = (root_ref.left.clone(), root_ref.right.clone());
        let (search_left, search_right) = (
            Self::lowest_common_ancestor(left_node, Some(p_node.clone()), Some(q_node.clone())),
            Self::lowest_common_ancestor(right_node, Some(p_node.clone()), Some(q_node.clone())),
        );

        match (search_left, search_right) {
            (None, some_right) => some_right,
            (some_left, None) => some_left,
           _ => Some(root_node.clone())
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
        fn helper(
            node: &Option<Rc<RefCell<TreeNode>>>,
            val1: i32,
            val2: i32,
            found1: &mut Option<Rc<RefCell<TreeNode>>>,
            found2: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = node {
                let n_borrow = n.borrow();
                if n_borrow.val == val1 {
                    *found1 = Some(Rc::clone(n));
                }
                if n_borrow.val == val2 {
                    *found2 = Some(Rc::clone(n));
                }
                // 两个都找到就提前返回
                if found1.is_some() && found2.is_some() {
                    return;
                }
                helper(&n_borrow.left, val1, val2, found1, found2);
                helper(&n_borrow.right, val1, val2, found1, found2);
            }
        }

        let mut found1 = None;
        let mut found2 = None;
        helper(root, val1, val2, &mut found1, &mut found2);
        (found1, found2)
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

    tree_ancestor_test!(test_child, (vec![Some(3),Some(5),Some(1),Some(6),Some(2),Some(0),Some(8),None,None,Some(7),Some(4)], 5, 4) => 5);
    tree_ancestor_test!(test_sibling, (vec![Some(3),Some(5),Some(1),Some(6),Some(2),Some(0),Some(8),None,None,Some(7),Some(4)], 5, 1) => 3);
    tree_ancestor_test!(test_simple, (vec![Some(1),Some(2)], 1, 2) => 1);
}
