// https://leetcode.com/problems/unique-paths

struct Solution {}

impl Solution {
    const MOD: i32 = 2 * 1_000_000_000;

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (n, m) = (n as usize, m as usize);

        let mut dp = vec![vec![0; m + 1]; n + 1];
        dp[0][1] = 1; // or dp[1][0]

        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % Solution::MOD;
            }
        }

        dp[n][m]
    }
}
