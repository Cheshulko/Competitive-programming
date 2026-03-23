// https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix

struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        const M: i64 = 1_000_000_000 + 7;

        let n = grid.len();
        let m = grid[0].len();

        let mut dp = vec![vec![[i64::MAX, i64::MIN]; m]; n];
        dp[0][0][0] = grid[0][0] as i64;
        dp[0][0][1] = grid[0][0] as i64;

        for i in 1..n {
            let v = grid[i][0] as i64;

            dp[i][0][0] = dp[i][0][0]
                .min(dp[i - 1][0][0] * v)
                .min(dp[i - 1][0][1] * v);

            dp[i][0][1] = dp[i][0][1]
                .max(dp[i - 1][0][0] * v)
                .max(dp[i - 1][0][1] * v);

            assert!(dp[i][0][0] <= dp[i][0][1]);
        }
        for j in 1..m {
            let v = grid[0][j] as i64;

            dp[0][j][0] = dp[0][j][0]
                .min(dp[0][j - 1][0] * v)
                .min(dp[0][j - 1][1] * v);

            dp[0][j][1] = dp[0][j][1]
                .max(dp[0][j - 1][0] * v)
                .max(dp[0][j - 1][1] * v);

            assert!(dp[0][j][0] <= dp[0][j][1]);
        }

        for i in 1..n {
            for j in 1..m {
                let v = grid[i][j] as i64;

                dp[i][j][0] = dp[i][j][0]
                    .min(dp[i - 1][j][0] * v)
                    .min(dp[i - 1][j][1] * v)
                    .min(dp[i][j - 1][0] * v)
                    .min(dp[i][j - 1][1] * v);

                dp[i][j][1] = dp[i][j][1]
                    .max(dp[i - 1][j][0] * v)
                    .max(dp[i - 1][j][1] * v)
                    .max(dp[i][j - 1][0] * v)
                    .max(dp[i][j - 1][1] * v);

                assert!(dp[i][j][0] <= dp[i][j][1]);
            }
        }

        (dp[n - 1][m - 1][1] % M).max(-1) as i32
    }
}
