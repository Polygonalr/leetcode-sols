/*
Approach: Greedy algorithm

Maintain two pointers starting from the left and right ends of the array. At every iteration, calculate current area
based on the heights of the two pointers. Move the pointer with the smaller height inwards until the two pointers meet
and do the same.

Repeat until the two pointers meet. Return the best area.
*/

use std::cmp::{min, max};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j): (usize, usize) = (0, height.len()-1 as usize);
        let mut best = 0;
        while i != j {
            best = max(best, (j-i) * min(height[i], height[j]) as usize);
            if height[i] < height[j] {
                let prev_height = height[i];
                while height[i] <= prev_height && i != j {
                    i += 1;
                }
            } else {
                let prev_height = height[j];
                while height[j] <= prev_height && i != j {
                    j -= 1;
                }
            }
        }
        best as i32
    }
}