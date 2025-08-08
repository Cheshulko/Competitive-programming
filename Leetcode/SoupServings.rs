// https://leetcode.com/problems/soup-servings

struct Solution {}

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let n = (n + 24) / 25;
        let n = 1000.min(n) as usize;

        let mut dp = vec![vec![0.0; n + 1]; n + 1];
        dp[n][n] = 1.0;

        for i in (1..=n).rev() {
            for j in (1..=n).rev() {
                dp[i.saturating_sub(4)][j.saturating_sub(0)] += 0.25 * dp[i][j];
                dp[i.saturating_sub(3)][j.saturating_sub(1)] += 0.25 * dp[i][j];
                dp[i.saturating_sub(2)][j.saturating_sub(2)] += 0.25 * dp[i][j];
                dp[i.saturating_sub(1)][j.saturating_sub(3)] += 0.25 * dp[i][j];
            }
        }

        let mut ans = 0.;
        for i in 1..=n {
            ans += dp[0][i];
        }
        ans += 0.5 * dp[0][0];

        ans
    }
}
