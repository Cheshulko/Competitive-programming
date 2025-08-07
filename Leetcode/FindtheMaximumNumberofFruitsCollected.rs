// https://leetcode.com/problems/find-the-maximum-number-of-fruits-collected

struct Solution {}

impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();

        let mut ch1 = 0;
        for i in 0..n {
            ch1 += fruits[i][i];
        }

        let ch2 = {
            let mut dp = vec![vec![i32::MIN; n + 2]; n + 2];
            dp[n][1] = 0;

            for j in 1..=n {
                for i in (1..=n).rev() {
                    if i <= j && (i != n && j != n) {
                        continue;
                    }

                    dp[i][j] = dp[i][j]
                        .max(dp[i - 1][j - 1])
                        .max(dp[i + 1][j - 1])
                        .max(dp[i][j - 1])
                        + fruits[i - 1][j - 1];
                }
            }

            dp[n][n]
        };

        let ch3 = {
            let mut dp = vec![vec![i32::MIN; n + 2]; n + 2];
            dp[1][n] = 0;

            for i in 1..=n {
                for j in (1..=n).rev() {
                    if i >= j && (i != n && j != n) {
                        continue;
                    }

                    dp[i][j] = dp[i][j]
                        .max(dp[i - 1][j - 1])
                        .max(dp[i - 1][j])
                        .max(dp[i - 1][j + 1])
                        + fruits[i - 1][j - 1];
                }
            }

            dp[n][n]
        };

        ch1 + ch2 + ch3 - 2 * fruits[n - 1][n - 1]
    }
}
