use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (m, n) = (maze.len() as i32, maze[0].len() as i32);
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut maze = maze; // 所有权转移
        let mut queue = VecDeque::new();

        queue.push_back((entrance[0], entrance[1], 0));
        maze[entrance[0] as usize][entrance[1] as usize] = '+';

        let is_border = |x, y, m, n| x == 0 || y == 0 || x == m - 1 || y == n - 1;

        while let Some((x, y, steps)) = queue.pop_front() {
            if is_border(x, y, m, n) && (x, y) != (entrance[0], entrance[1]) {
                return steps;
            }

            for (dx, dy) in directions.iter() {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0 && ny >= 0 && nx < m && ny < n && maze[nx as usize][ny as usize] == '.' {
                    maze[nx as usize][ny as usize] = '+';
                    queue.push_back((nx, ny, steps + 1));
                }
            }
        }

        -1
    }
}
