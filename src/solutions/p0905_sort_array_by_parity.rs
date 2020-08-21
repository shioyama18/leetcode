struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let (mut left, mut right) = (0, a.len() - 1);

        while left < right {
            if a[left] % 2 == 1 {
                a.swap(left, right);
                right -= 1;
            } else {
                left += 1;
            }
        }

        a
    }
}
