struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.chars()
            .rev()
            .enumerate()
            .map(|(i, c)| (c as i32 - 64) * 26i32.pow(i as u32))
            .sum()
    }
}
