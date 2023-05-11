impl Solution {
    fn letter_to_idx(c: char) -> usize {
        if c as usize <= 90 {
            c as usize - 65
        } else {
            c as usize - 97 + 26
        }
    }

    pub fn longest_palindrome(s: String) -> i32 {
        let mut memo = vec![0; 26*2];
        let mut total_chars = 0;
        for c in s.chars() {
            memo[Solution::letter_to_idx(c)] += 1;
            total_chars += 1;
        }
        let mut total_len = 0;
        for count in memo {
            let d = count / 2;
            total_len += d * 2;
            total_chars -= d * 2;
        }
        if total_chars >= 1 {
            total_len += 1;
        }
        total_len
    }
}
