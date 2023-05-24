/*
Approach: BFS to discover entire islands

Iterate through every cell. If cell is '1', recursively discover its neighbours until all the '1' cells belonging to the
same island are discovered. Set these cells to '9' to mark them as discovered island. Increment the island counter
afterwards.

Runtime: O(n) due to iterating through all cells at most twice.

Note: arrow anti pattern here is apparent and horrid
*/

use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut g: Vec<Vec<char>> = grid.clone();
        let mut count = 0;
        for row in 0..g.len() as i32 {
            for col in 0..g[0].len() as i32 {
                if g[row as usize][col as usize] == '1' {
                    let mut queue: VecDeque<(i32, i32)> = VecDeque::from(vec![(row, col)]);
                    count += 1;
                    g[row as usize][col as usize] = '9';
                    while let Some((x, y)) = queue.pop_front() {
                        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                            if x + dx < 0 || x + dx >= g.len() as i32
                                || y + dy < 0 || y + dy >= g[0].len() as i32 {
                                continue;
                            }

                            let (nx, ny): (i32, i32) = (x + dx, y + dy);

                            if g[nx as usize][ny as usize] == '1' {
                                g[nx as usize][ny as usize] = '9';
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }
            }
        }
        count
    }
}