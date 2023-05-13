// https://leetcode.com/problems/count-ways-to-build-good-strings

struct Solution {}

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let modulo = 1_000_000_000 + 7;
        let (low, high) = (low as usize, high as usize);
        let (zero, one) = (zero as usize, one as usize);

        let mut ans = 0;
        let mut dp = vec![0; high + 1];

        dp[0] = 1;

        for len in 0..(high + 1) {
            let add_zeros = len + zero;
            let add_ones = len + one;

            if add_zeros <= high {
                dp[add_zeros] = (dp[add_zeros] + dp[len]) % modulo;
            }
            if add_ones <= high {
                dp[add_ones] = (dp[add_ones] + dp[len]) % modulo;
            }

            if len >= low {
                ans = (ans + dp[len]) % modulo;
            }
        }

        ans
    }
}
