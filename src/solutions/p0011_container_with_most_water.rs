struct Solution;

use std::cmp::{max, min};
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            let area = min(height[left], height[right]) * (right - left) as i32;
            max_area = max(max_area, area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}
