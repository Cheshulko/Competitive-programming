// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings

struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.into_bytes().into_iter().collect::<Vec<_>>();
        let s2 = s2.into_bytes().into_iter().collect::<Vec<_>>();
        let n1 = s1.len();
        let n2 = s2.len();

        let mut dp = vec![vec![i32::MAX; n2 + 1]; n1 + 1];
        dp[0][0] = 0;
        for i in 0..n1 {
            dp[i + 1][0] = dp[i][0] + s1[i] as i32;
        }
        for j in 0..n2 {
            dp[0][j + 1] = dp[0][j] + s2[j] as i32;
        }

        for i in 0..n1 {
            for j in 0..n2 {
                dp[i + 1][j + 1] = dp[i + 1][j + 1]
                    .min(dp[i][j] + (s1[i] + s2[j]) as i32 * (s1[i] != s2[j]) as i32);
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j + 1] + s1[i] as i32);
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i + 1][j] + s2[j] as i32);
            }
        }

        dp[n1][n2]
    }
}
