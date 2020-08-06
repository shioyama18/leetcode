struct Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        (num > 0) && ((num & (num - 1)) == 0) && ((num & 0b1010101010101010101010101010101) == 0)
    }
}
