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
use std::collections::VecDeque;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn helper(
            root: Option<Rc<RefCell<TreeNode>>>,
            deque: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Vec<Option<i32>> {
            deque.push_back(root);
            let mut result = vec![];
            while let Some(head) = deque.pop_front() {
                if let Some(node) = head {
                    let borrow = node.borrow();

                    result.push(Some(borrow.val));

                    if borrow.left.is_some() || borrow.right.is_some() {
                        deque.push_back(borrow.left.clone());
                        deque.push_back(borrow.right.clone());
                    }
                } else {
                    result.push(None);
                }
            }
            result
        }

        let mut deque = VecDeque::new();
        let list = helper(root, &mut deque);

        let elements: Vec<String> = list
            .iter()
            .map(|&val| {
                if let Some(i) = val {
                    i.to_string()
                } else {
                    "null".to_string()
                }
            })
            .collect();
        elements.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let list: Vec<Option<i32>> = data
            .split(",")
            .map(|s| {
                if let Ok(i) = s.parse::<i32>() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        if list.is_empty() || list.get(0).is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(list[0]?)));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;

        while i < list.len() {
            let node = queue.pop_front().unwrap();
            // Left child
            if i < list.len() {
                if let Some(val) = list[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            // Right child
            if i < list.len() {
                if let Some(val) = list[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
#[cfg(test)]
mod tests {
    use super::*;

    fn from_unbalanced_vec(
        data: &[Option<i32>],
    ) -> Option<Rc<RefCell<TreeNode>>> {
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
    macro_rules! tree_serialization_test {
        ($name: ident, $input: expr) => {
            #[test]
            fn $name() {
                let original: Vec<Option<i32>> = $input;

                let obj = Codec::new();
                let root: Option<Rc<RefCell<TreeNode>>> =
                    from_unbalanced_vec(&original);
                let data: String = obj.serialize(root.clone());
                let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
                assert_eq!(root, ans);
            }
        };
    }

    tree_serialization_test!(empty, vec![]);
    tree_serialization_test!(
        left_none,
        vec![Some(1), None, Some(3), None, Some(4), Some(5)]
    );
    tree_serialization_test!(
        test_right_left,
        vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
            Some(6),
            Some(7)
        ]
    );
}
