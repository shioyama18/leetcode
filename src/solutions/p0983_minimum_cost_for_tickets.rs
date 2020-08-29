struct Solution;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let days: HashSet<i32> = HashSet::from_iter(days);
        let costs = costs.into_iter().zip(vec![1, 7, 30]).collect::<Vec<_>>();
        let mut cache = HashMap::new();

        Self::dp(&days, &costs, &mut cache, 1)
    }

    fn dp(days: &HashSet<i32>, costs: &[(i32, i32)], cache: &mut HashMap<i32, i32>, i: i32) -> i32 {
        if cache.contains_key(&i) {
            return cache[&i];
        }

        if i > 365 {
            return 0;
        }

        let min_cost = if days.contains(&i) {
            let mut tmp = std::i32::MAX;
            for (c, d) in costs.iter() {
                tmp = std::cmp::min(tmp, Self::dp(days, costs, cache, i + d) + c);
            }
            tmp
        } else {
            Self::dp(days, costs, cache, i + 1)
        };
        cache.insert(i, min_cost);
        min_cost
    }
}
