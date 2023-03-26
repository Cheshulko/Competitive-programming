// https://leetcode.com/problems/cherry-pickup-ii

impl Solution {
    const DIRS_Y: [i32; 3] = [0, -1, 1];

    fn can(to_j: i32, m: usize) -> bool {
        to_j >= 0 && to_j < m as i32
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut ans = 0;
        let mut dp = vec![[[-1_000_000; 71]; 71]; 71];

        dp[0][0][m - 1] = grid[0][0] + grid[0][m - 1];

        for i in 0..n - 1 {
            for j1 in 0..m {
                for j2 in 0..m {
                    let cur = dp[i][j1][j2];

                    for d1 in Solution::DIRS_Y {
                        for d2 in Solution::DIRS_Y {
                            let to_i = i + 1;
                            let to_j1 = j1 as i32 + d1;
                            let to_j2 = j2 as i32 + d2;

                            let to_j1_u = to_j1 as usize;
                            let to_j2_u = to_j2 as usize;

                            if Solution::can(to_j1, m) && Solution::can(to_j2, m) {
                                let upd: i32;
                                if to_j1 == to_j2 {
                                    upd = grid[to_i][to_j1_u];
                                } else {
                                    upd = grid[to_i][to_j1_u] + grid[to_i][to_j2_u];
                                }

                                dp[to_i][to_j1_u][to_j2_u] =
                                    std::cmp::max(dp[to_i][to_j1_u][to_j2_u], cur + upd);
                            }
                        }
                    }
                }
            }
        }

        for j1 in 0..m {
            for j2 in 0..m {
                ans = std::cmp::max(ans, dp[n - 1][j1][j2]);
            }
        }

        ans
    }
}
