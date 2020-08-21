struct Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let n = num_people as usize;
        let mut output = vec![0; n];
        let mut candies = candies;
        let mut i = 0;
        while candies > 0 {
            let c = std::cmp::min(candies, i as i32 + 1);
            output[i % n] += c;
            candies -= c;
            i += 1;
        }

        output
    }
}
