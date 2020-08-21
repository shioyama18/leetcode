struct Solution;

impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let mut output = vec![];

        for (i, w) in s.split_whitespace().enumerate() {
            let mut w = w.chars().collect::<Vec<char>>();
            if !"aoieuAOIEU".contains(w[0]) {
                w.rotate_left(1);
            }
            w.push('m');
            w.push('a');
            for _ in 0..=i {
                w.push('a');
            }

            output.push(w.into_iter().collect::<String>());
        }

        output.join(" ")
    }
}
