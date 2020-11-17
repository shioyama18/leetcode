struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut number = vec![];

        for (i, c) in str.trim().chars().enumerate() {
            if !c.is_digit(10) {
                if i != 0 || c != '-' {
                    break;
                }
                if c == '+' {
                    continue;
                }
            }
            number.push(c.to_string())
        }

        if let Ok(n) = number.join("").parse::<i32>() {
            return n;
        }

        if number.len() <= 1 {
            0
        } else if number[0] == "-" {
            std::i32::MIN
        } else {
            std::i32::MAX
        }
    }
}
