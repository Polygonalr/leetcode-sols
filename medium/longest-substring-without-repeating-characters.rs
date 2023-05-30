/*
Approach: Sliding window

Maintain a dictionary of characters with their last seen index (initialized to -1 at the start). Slide over the string
and update the dictionary entry of the current character with the current index. If the current character was seen
before, set the start of the window to the max(last seen idx of the current character, start idx of current window).
This ensures that the current window will not contain any duplicate characters.

Update the best length of the window at every iteration. Return the best length at the end.

Runtime: O(n)
*/

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s == "" {
            return 0;
        }
        let mut memo = vec![-1; 128];
        let (mut start, mut best) = (0, -1);
        for (i, c) in s.chars().enumerate() {
            let idx = c as usize;
            if memo[idx] != -1 {
                start = std::cmp::max(memo[idx] + 1, start);
            }
            memo[idx] = i as i32;
            best = std::cmp::max(best, i as i32 - start + 1);
        }
        best
    }
}