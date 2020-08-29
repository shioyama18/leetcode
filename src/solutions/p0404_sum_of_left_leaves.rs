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

struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            queue.push_back((node, false));
        } else {
            return 0;
        }

        while let Some((node, is_left)) = queue.pop_front() {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            match (left, right) {
                (Some(l), Some(r)) => {
                    queue.push_back((l, true));
                    queue.push_back((r, false));
                }
                (Some(l), None) => {
                    queue.push_back((l, true));
                }
                (None, Some(r)) => {
                    queue.push_back((r, false));
                }
                (None, None) => {
                    if is_left {
                        total += node.borrow().val;
                    }
                }
            }
        }

        total
    }
}
