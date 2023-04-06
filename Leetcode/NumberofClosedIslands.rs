// https://leetcode.com/problems/number-of-closed-islands

struct Solution {}

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: usize, j: usize, grid: &Vec<Vec<i32>>, used: &mut Vec<Vec<bool>>, ok: &mut bool) {
            used[i][j] = true;
            if (i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1)
                && grid[i][j] == 0
            {
                *ok = false;
            }
            for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let toi = i as i32 + di;
                let toj = j as i32 + dj;
                if toi >= 0
                    && toi < grid.len() as i32
                    && toj >= 0
                    && toj < grid[0].len() as i32
                    && grid[toi as usize][toj as usize] == 0
                    && !used[toi as usize][toj as usize]
                {
                    dfs(toi as usize, toj as usize, grid, used, ok);
                }
            }
        }

        let mut used: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut ans = 0;
        for i in 1..grid.len() {
            for j in 1..grid[i].len() {
                if !used[i][j] && grid[i][j] == 0 {
                    let mut ok = true;
                    dfs(i, j, &grid, &mut used, &mut ok);
                    if ok {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
