struct Solution;

impl Solution {
    pub fn find_permutation(s: String) -> Vec<i32> {
        let s = s.chars().collect::<Vec<_>>();
        let mut stack = vec![];
        let mut output = vec![];

        for i in 1..=s.len() + 1 {
            stack.push(i);
            if i == s.len() + 1 || s[i - 1] == 'I' {
                while let Some(j) = stack.pop() {
                    output.push(j as i32);
                }
            }
        }

        output
    }
}
