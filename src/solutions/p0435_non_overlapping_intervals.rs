struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_by_key(|interval| get_end(interval));
        let mut end = get_end(&intervals[0]);
        let mut count = 0;
        for i in 1..intervals.len() {
            if get_start(&intervals[i]) < end {
                // Overlapping so increment count and skip interval
                count += 1;
            } else {
                // Not overlapping so update end value
                end = get_end(&intervals[i])
            }
        }

        count
    }
}

fn get_start(interval: &[i32]) -> i32 {
    interval[0]
}

fn get_end(interval: &[i32]) -> i32 {
    interval[1]
}
