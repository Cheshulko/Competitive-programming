// https://leetcode.com/problems/number-of-increasing-paths-in-a-grid

struct Solution {}

impl Solution {
    const MODULO: i32 = 1_000_000_000 + 7;

    fn dfs(
        i: usize,
        j: usize,
        n: i32,
        m: i32,
        grid: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if dp[i][j] != -1 {
            return dp[i][j];
        }

        let cur = grid[i][j];
        let mut ans = 1;

        for (di, dj) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let to_i = i as i32 + di;
            let to_j = j as i32 + dj;

            if to_i >= 0
                && to_i < n
                && to_j >= 0
                && to_j < m
                && cur < grid[to_i as usize][to_j as usize]
            {
                ans = (ans + Solution::dfs(to_i as usize, to_j as usize, n, m, grid, dp))
                    % Solution::MODULO;
            }
        }

        dp[i][j] = ans;

        ans
    }

    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; m]; n];

        let mut ans = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                ans = (ans + Solution::dfs(i, j, n as i32, m as i32, &grid, &mut dp))
                    % Solution::MODULO;
            }
        }

        ans
    }
}
