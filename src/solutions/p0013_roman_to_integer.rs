struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let romans = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ];
        let romans = romans.into_iter().collect::<HashMap<_, _>>();
        let s = s.chars().collect::<Vec<_>>();
        let mut output = 0;

        for i in (0..s.len()).rev() {
            if i != s.len() - 1 && romans[&s[i]] < romans[&s[i + 1]] {
                output -= romans[&s[i]];
            } else {
                output += romans[&s[i]];
            }
        }

        output
    }
}
