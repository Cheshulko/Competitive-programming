// https://leetcode.com/problems/unique-paths-ii

struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();
        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = (obstacle_grid[0][0] != 1) as i32;

        for i in 0..n {
            for j in 0..m {
                if i + 1 < n && obstacle_grid[i + 1][j] != 1 {
                    dp[i + 1][j] += dp[i][j];
                }
                if j + 1 < m && obstacle_grid[i][j + 1] != 1 {
                    dp[i][j + 1] += dp[i][j];
                }
            }
        }

        dp[n - 1][m - 1]
    }
}
