struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i..n {
                let i_j = matrix[i][j];
                let j_i = matrix[j][i];
                matrix[i][j] = j_i;
                matrix[j][i] = i_j;
            }
        }

        for i in 0..n {
            matrix[i].reverse();
        }
    }
}
