struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let romans = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut output = vec![];
        let mut num = num;

        for (n, r) in romans.into_iter() {
            while num >= n {
                output.push(r);
                num -= n;
            }
        }

        output.join("")
    }
}
