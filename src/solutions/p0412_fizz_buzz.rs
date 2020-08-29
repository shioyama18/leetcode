struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut output = vec![];

        for i in 1..=n {
            match i {
                x if x % 15 == 0 => {
                    output.push("FizzBuzz".to_string());
                }
                x if x % 5 == 0 => {
                    output.push("Buzz".to_string());
                }
                x if x % 3 == 0 => {
                    output.push("Fizz".to_string());
                }
                x => {
                    output.push(x.to_string());
                }
            }
        }

        output
    }
}
