struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut starts = BTreeMap::new();
        let mut output = vec![-1i32; intervals.len()];

        for i in 0..intervals.len() {
            starts.insert(intervals[i][0], i as i32);
        }

        for i in 0..intervals.len() {
            let mut iter = starts.keys().skip_while(|&&k| k < intervals[i][1]);
            if let Some(pos) = iter.next() {
                output[i] = starts[pos];
            }
        }

        output
    }
}
