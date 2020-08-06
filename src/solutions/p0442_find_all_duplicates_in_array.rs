struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![];

        for i in 0..nums.len() {
            let x = nums[i].abs() as usize;
            if nums[x - 1] < 0 {
                output.push(x as i32);
            }
            nums[x - 1] *= -1;
        }

        output
    }
}
