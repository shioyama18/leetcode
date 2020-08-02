struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.chars().all(|c| c.is_uppercase()) || word.chars().all(|c| c.is_lowercase()) {
            return true;
        }

        for (i, c) in word.chars().enumerate() {
            if i == 0 && c.is_lowercase() {
                return false;
            }
            if i != 0 && c.is_uppercase() {
                return false;
            }
        }

        true
    }
}
