// https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k

struct Solution {}

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let k = k as usize;
        let n = grid.len();
        let m = grid[0].len();

        let mut dp = vec![vec![vec![0; k]; m + 1]; n + 1];
        dp[1][1][grid[0][0] as usize % k] = 1;

        for i in 1..=n {
            for j in 1..=m {
                let v = grid[i - 1][j - 1] as usize;
                for r in 0..k {
                    dp[i][j][(r + v) % k] += dp[i - 1][j][r];
                    dp[i][j][(r + v) % k] += dp[i][j - 1][r];
                    dp[i][j][(r + v) % k] %= M;
                }
            }
        }

        dp[n][m][0] as i32
    }
}
