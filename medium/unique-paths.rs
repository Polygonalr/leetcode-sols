/*
Permutation question on how many permutations of (m-1) rights and (n-1) downs.
Formula is (m+n-2)! / (m-1)! (n-1)! Below code is optimized for minimal calculation required for the factorials.
*/
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 1 || n <= 1 {
            return 1;
        }
        let a = std::cmp::max(m, n);
        let b = std::cmp::min(m, n);

        (Solution::partial_factorial(a-1, a+b-2) / Solution::factorial(b-1)) as i32
    }

    fn factorial(s: i32) -> i128 {
        let mut ans: i128 = 1;
        for i in 2..=s as i128 {
            ans *= i;
        }
        ans
    }

    fn partial_factorial(f: i32, t: i32) -> i128 {
        let mut ans: i128 = 1;
        for i in (f+1) as i128..=t as i128 {
            ans *= i;
        }
        ans
    }
}