struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }

        let mut kth_row = vec![];

        for _ in 1..=row_index {
            let prev = kth_row;
            kth_row = vec![1];
            for w in prev.windows(2) {
                kth_row.push(w.iter().sum());
            }
            kth_row.push(1);
        }

        kth_row
    }
}
