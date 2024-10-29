// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid

struct Solution {}

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut dp = vec![vec![i32::MIN; m]; n];
        for i in 0..n {
            dp[i][0] = 0;
        }

        let mut ans = 0;
        for j in 1..m {
            for i in 0..n {
                if grid[i][j - 1] < grid[i][j] && i32::MIN != dp[i][j - 1] {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1] + 1);
                }
                if i > 0 && grid[i - 1][j - 1] < grid[i][j] && i32::MIN != dp[i - 1][j - 1] {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
                }
                if i + 1 < n && grid[i + 1][j - 1] < grid[i][j] && i32::MIN != dp[i + 1][j - 1] {
                    dp[i][j] = dp[i][j].max(dp[i + 1][j - 1] + 1);
                }
                ans = ans.max(dp[i][j]);
            }
        }

        ans
    }
}
