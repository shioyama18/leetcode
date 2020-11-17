struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mapping = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            let comp = target - n;

            if mapping.contains_key(&comp) {
                return vec![mapping[&comp], i as i32];
            }

            mapping.insert(n, i as i32);
        }

        unreachable!()
    }
}
