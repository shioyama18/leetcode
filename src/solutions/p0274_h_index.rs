struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by_key(|c| -c);
        let mut h_index = 0;

        for _ in citations
            .iter()
            .enumerate()
            .take_while(|(i, &c)| c >= *i as i32 + 1)
        {
            h_index += 1;
        }

        h_index
    }
}
