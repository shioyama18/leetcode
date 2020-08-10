struct Solution;

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
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut cols: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));

        while let Some((node, col)) = queue.pop_front() {
            if let Some(node) = node {
                cols.entry(col)
                    .or_insert(Vec::new())
                    .push(node.borrow().val);

                queue.push_back((node.borrow_mut().left.take(), col - 1));
                queue.push_back((node.borrow_mut().right.take(), col + 1));
            }
        }

        cols.values().cloned().collect()
    }
}
