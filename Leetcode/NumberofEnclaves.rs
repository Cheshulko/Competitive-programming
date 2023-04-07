// https://leetcode.com/problems/number-of-enclaves

struct Solution {}

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            i: usize,
            j: usize,
            grid: &Vec<Vec<i32>>,
            used: &mut Vec<Vec<bool>>,
            ok: &mut bool,
            depth: &mut i32,
        ) {
            used[i][j] = true;
            if (i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1)
                && grid[i][j] == 1
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
                    && grid[toi as usize][toj as usize] == 1
                    && !used[toi as usize][toj as usize]
                {
                    *depth += 1;
                    dfs(toi as usize, toj as usize, grid, used, ok, depth);
                }
            }
        }

        let mut used: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut ans = 0;
        for i in 1..grid.len() {
            for j in 1..grid[i].len() {
                if !used[i][j] && grid[i][j] == 1 {
                    let mut ok = true;
                    let mut depth = 1;
                    dfs(i, j, &grid, &mut used, &mut ok, &mut depth);
                    if ok {
                        ans += depth
                    }
                }
            }
        }
        ans
    }
}
