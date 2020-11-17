struct Solution;

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut longest = 0;
        let mut found = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            if let Some(&index) = found.get(&c) {
                start = max(start, index);
            }
            longest = max(longest, i - start + 1);
            found.insert(c, i);
        }

        longest as i32
    }
}
