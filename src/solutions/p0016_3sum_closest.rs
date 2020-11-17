struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut closest = 20000;
        let n = nums.len();

        for i in 0..n - 2 {
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let total = nums[i] + nums[l] + nums[r];
                if (target - total).abs() < (target - closest).abs() {
                    closest = total;
                }

                if total < target {
                    l += 1;
                } else if total > target {
                    r -= 1;
                } else {
                    return target;
                }
            }
        }

        closest
    }
}
