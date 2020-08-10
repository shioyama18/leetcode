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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut k = sum;
        let mut count = 0;
        let mut h = HashMap::new();
        Solution::preorder(root, 0, &mut k, &mut count, &mut h);
        count
    }

    fn preorder(
        node: Option<Rc<RefCell<TreeNode>>>,
        mut curr_sum: i32,
        k: &mut i32,
        count: &mut i32,
        h: &mut HashMap<i32, i32>,
    ) {
        if let Some(n) = node {
            curr_sum += n.borrow().val;
            if curr_sum == *k {
                *count += 1;
            }

            *count += h.get(&(curr_sum - *k)).unwrap_or(&0);
            h.insert(curr_sum, h.get(&curr_sum).unwrap_or(&0) + 1);
            Solution::preorder(n.borrow_mut().left.take(), curr_sum, k, count, h);
            Solution::preorder(n.borrow_mut().right.take(), curr_sum, k, count, h);
            h.insert(curr_sum, h.get(&curr_sum).unwrap() - 1);
        }
    }
}
