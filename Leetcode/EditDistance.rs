// https://leetcode.com/problems/edit-distance

struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();

        let w1_len = w1.len();
        let w2_len = w2.len();

        let mut dp = vec![vec![i32::MAX - 1; w2_len + 1]; w1_len + 1];
        dp[0][0] = 0;

        for i in 1..=w1_len {
            dp[i][0] = i as i32;
        }

        for j in 1..=w2_len {
            dp[0][j] = j as i32;
        }

        for i in 1..=w1_len {
            for j in 1..=w2_len {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + (w1[i - 1] != w2[j - 1]) as i32);
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            }
        }

        dp[w1_len][w2_len]
    }
}
