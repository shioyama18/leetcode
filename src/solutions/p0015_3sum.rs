struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n <= 2 {
            return vec![];
        }

        nums.sort();
        let mut output = vec![];

        for i in 0..n - 2 {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                let total = nums[i] + nums[left] + nums[right];
                if total < 0 {
                    left += 1;
                } else if total > 0 {
                    right -= 1;
                } else {
                    output.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }

        output
    }
}
