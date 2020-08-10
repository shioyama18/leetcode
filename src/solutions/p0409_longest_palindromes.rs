struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counter = HashMap::new();
        let mut total = 0;

        for c in s.chars() {
            counter.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }

        for n in counter.values() {
            total += n / 2 * 2;

            if total % 2 == 0 && n % 2 == 1 {
                total += 1;
            }
        }

        total
    }
}
