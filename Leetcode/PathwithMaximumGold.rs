// https://leetcode.com/problems/path-with-maximum-gold

struct Solution {}

impl Solution {
    fn dfs((i, j): (usize, usize), grid: &Vec<Vec<i32>>, used: &mut Vec<Vec<bool>>) -> i32 {
        used[i][j] = true;

        let i_ = i as i32;
        let j_ = j as i32;

        let mut ans = grid[i][j];
        for (w, to_i, to_j) in
            [(0, 1), (-1, 0), (0, -1), (1, 0)]
                .into_iter()
                .filter_map(|(di, dj)| {
                    let to_i = (i_ + di) as usize;
                    let to_j = (j_ + dj) as usize;
                    let w = *grid.get(to_i)?.get(to_j)?;

                    (w > 0).then_some((w, to_i, to_j))
                })
        {
            if w > 0 && !used[to_i][to_j] {
                ans = ans.max(grid[i][j] + Solution::dfs((to_i, to_j), grid, used));
            }
        }

        used[i][j] = false;
        ans
    }
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut used = vec![vec![false; m]; n];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    used[i][j] = true;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] != 0 {
                    ans = ans.max(Solution::dfs((i, j), &grid, &mut used));
                }
            }
        }

        ans
    }
}
