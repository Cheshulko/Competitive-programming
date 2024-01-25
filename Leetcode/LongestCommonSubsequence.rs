// https://leetcode.com/problems/longest-common-subsequence

struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();

        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();

        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i][j]
                    .max(dp[i - 1][j])
                    .max(dp[i][j - 1])
                    .max(dp[i - 1][j - 1] + (text1[i - 1] == text2[j - 1]) as i32);
            }
        }

        dp[n][m]
    }
}
