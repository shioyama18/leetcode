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
use std::rc::Rc;

impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        let mut p = root;
        let mut closest = p.as_ref().unwrap().borrow().val;

        while let Some(curr) = p {
            let (diff_closest, diff_curr) = (
                (target - closest as f64).abs(),
                (target - curr.borrow().val as f64).abs(),
            );
            if diff_curr < diff_closest {
                closest = curr.borrow().val;
            }
            p = if curr.borrow().val as f64 > target {
                curr.borrow_mut().left.take()
            } else {
                curr.borrow_mut().right.take()
            };
        }

        closest
    }
}
