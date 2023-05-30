/*
Approach: Round-based simulation

Store coordinates of rotten oranges in a queue. For every round, pop every rotten orange from the queue and rot
its neighbours. If neighbour was a fresh orange, push it to the new queue. Repeat until there is a round where there are
no more fresh oranges (return no. of rounds) or there is a round where there are no new rotten oranges (return -1).
*/

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh = 0;
        let mut rotten_q: VecDeque<(usize, usize)> = VecDeque::new();

        for row_no in 0..grid.len() {
            for col_no in 0..grid[0].len() {
                match grid[row_no][col_no] {
                    2 => {
                        rotten_q.push_back((row_no, col_no));
                    }
                    1 => {
                        fresh += 1;
                    }
                    0 => {
                        continue;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        let mut rounds = 0;
        let mut prev_fresh = fresh;
        let mut next_rotten_q: VecDeque<(usize, usize)> = VecDeque::new();
        while fresh != 0 {
            while let Some((x, y)) = rotten_q.pop_front() {
                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if x as i32 + dx < 0 || x as i32 + dx >= grid.len() as i32
                        || y as i32 + dy < 0 || y as i32 + dy >= grid[0].len() as i32 {
                            continue;
                    }
                    let nx: usize = (x as i32 + dx) as usize;
                    let ny: usize = (y as i32 + dy) as usize;

                    if grid[nx][ny] == 1 {
                        grid[nx][ny] = 2;
                        next_rotten_q.push_back((nx, ny));
                        fresh -= 1;
                    }
                }
            }
            if fresh == prev_fresh {
                return -1;
            }
            rotten_q = next_rotten_q;
            prev_fresh = fresh;
            next_rotten_q = VecDeque::new();
            rounds += 1;
        }
        rounds
    }
}