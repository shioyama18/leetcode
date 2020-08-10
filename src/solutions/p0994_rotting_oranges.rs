struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut fresh = 0;
        let (rows, cols) = (grid.len(), grid[0].len());
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 2 {
                    queue.push_back((r, c));
                }
                if grid[r][c] == 1 {
                    fresh += 1;
                }
            }
        }

        queue.push_back((10, 10));
        let mut elapsed = -1;
        let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        while let Some((row, col)) = queue.pop_front() {
            if row == 10 {
                elapsed += 1;
                if !queue.is_empty() {
                    queue.push_back((10, 10));
                }
            } else {
                for (dr, dc) in directions.iter() {
                    let (r, c) = (row as i32 + dr, col as i32 + dc);
                    if r < 0 || c < 0 {
                        continue;
                    }
                    let (r, c) = (r as usize, c as usize);
                    if r < rows && c < cols && grid[r][c] == 1 {
                        grid[r][c] = 2;
                        fresh -= 1;
                        queue.push_back((r, c));
                    }
                }
            }
        }

        if fresh == 0 {
            elapsed
        } else {
            -1
        }
    }
}
