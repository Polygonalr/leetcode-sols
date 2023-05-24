/*
Approach: BFS starting from every 0 cell.

Create a new matrix of the same size with -1 as default value. Discover the location of each 0 cell and set the
corresponding cell in the new matrix to 0 as well. Store the coordinates of each 0 cell and push them to a queue with a
value 0.

For every entry in the queue, iterate through every neighbour. If a neighbour is uninitialized in the new matrix (-1),
set it to current value + 1 and push the coordinates of the neighbour into the queue with value = current value + 1.
Repeat until queue is empty, which means the new matrix is fully initialized.

Runtime: O(n) worst case runtime where every cell is 0 cell
*/

use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if mat.is_empty() {
            return vec![vec![]];
        }
        let (rows, cols) = (mat.len(), mat[0].len());
        let mut new_mat: Vec<Vec<i32>> = vec![vec![-1; cols]; rows];
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        for row_no in 0..rows {
            for col_no in 0..cols {
                if mat[row_no][col_no] == 0 {
                    queue.push_back((row_no, col_no, 0));
                    new_mat[row_no][col_no] = 0;
                }
            }
        }

        while let Some((x, y, val)) = queue.pop_front() {
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if x as i32 + dx < 0 || x as i32 + dx >= rows as i32
                    || y as i32 + dy < 0 || y as i32 + dy >= cols as i32 {
                        continue;
                }

                let (nx, ny): (usize, usize) = (
                    (x as i32 + dx) as usize,
                    (y as i32 + dy) as usize
                );
                if new_mat[nx][ny] == -1 {
                    new_mat[nx][ny] = val + 1;
                    queue.push_back((nx, ny, val + 1));
                }
            }
        }
        new_mat
    }
}