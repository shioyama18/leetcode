struct Solution;

use std::cmp::{max, min};
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut t1_cost, mut t2_cost) = (std::i32::MAX, std::i32::MAX);
        let (mut t1_profit, mut t2_profit) = (0, 0);

        for p in prices.into_iter() {
            t1_cost = min(t1_cost, p);
            t1_profit = max(t1_profit, p - t1_cost);
            t2_cost = min(t2_cost, p - t1_profit);
            t2_profit = max(t2_profit, p - t2_cost);
        }

        t2_profit
    }
}
