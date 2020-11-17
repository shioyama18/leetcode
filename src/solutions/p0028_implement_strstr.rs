struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.chars().collect::<Vec<_>>();
        let needle = needle.chars().collect::<Vec<_>>();
        let n = needle.len();

        if n == 0 {
            return 0;
        }

        for i in 0..(haystack.len() - n + 1) {
            if &haystack[i..i + n] == &needle[..] {
                return i as i32;
            }
        }

        -1
    }
}
