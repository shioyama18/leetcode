struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        assert!(!maze.is_empty());
        let mut visited = vec![vec![false; maze[0].len()]; maze.len()];

        assert!(start.iter().all(|&i| i >= 0));
        let start = start.into_iter().map(|i| i as usize).collect::<Vec<_>>();
        let destination = (destination[0] as usize, destination[1] as usize);

        let mut queue = VecDeque::new();
        queue.push_back((start[0], start[1]));
        visited[start[0]][start[1]] = true;
        let directions = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

        while let Some((x, y)) = queue.pop_back() {
            if (x, y) == destination {
                return true;
            }

            for (dx, dy) in directions.iter() {
                let (mut x, mut y) = (x as i32 + dx, y as i32 + dy);
                while x >= 0
                    && x < maze.len() as i32
                    && y >= 0
                    && y < maze[0].len() as i32
                    && maze[x as usize][y as usize] == 0
                {
                    x += dx;
                    y += dy;
                }

                let x = (x - dx) as usize;
                let y = (y - dy) as usize;
                if !visited[x][y] {
                    visited[x][y] = true;
                    queue.push_back((x, y));
                }
            }
        }

        false
    }
}
