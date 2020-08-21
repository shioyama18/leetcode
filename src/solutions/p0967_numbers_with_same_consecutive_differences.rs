struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            return (0..=9).collect();
        }

        let mut output = (1..=9).collect::<Vec<_>>();

        for _ in 0..n - 1 {
            let mut next = vec![];
            for num in output.into_iter() {
                let tail_digit = num % 10;
                let next_digits = if k != 0 {
                    vec![tail_digit + k, tail_digit - k]
                } else {
                    vec![tail_digit]
                };

                for x in next_digits.into_iter() {
                    if x >= 0 && x < 10 {
                        next.push(num * 10 + x);
                    }
                }
            }
            output = next;
        }

        output
    }
}
