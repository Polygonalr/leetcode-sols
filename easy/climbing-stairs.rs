impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = vec![-1; (n+1) as usize];
        memo[0] = 1;
        memo[1] = 1;
        for i in 2..=(n as usize) {
            memo[i] = memo[i-1] + memo[i-2];
        }
        return memo[n as usize];
    }
}
