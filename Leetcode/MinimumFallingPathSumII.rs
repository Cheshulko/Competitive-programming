// https://leetcode.com/problems/minimum-falling-path-sum-ii

struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut dp = vec![vec![i32::MAX; m]; n + 1];

        for j in 0..m {
            dp[0][j] = grid[0][j];
        }

        for i in 1..n {
            for j in 0..m {
                for k in 0..m {
                    if j == k {
                        continue;
                    }

                    dp[i][j] = dp[i][j].min(dp[i - 1][k] + grid[i][j]);
                }
            }
        }

        *dp[n - 1].iter().min().unwrap()
    }
}
