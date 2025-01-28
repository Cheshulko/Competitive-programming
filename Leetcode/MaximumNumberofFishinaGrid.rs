// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid

struct Solution {}

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
        let n = grid.len();
        let m = grid[0].len();

        fn dfs((i, j): (usize, usize), grid: &mut Vec<Vec<i32>>) -> i32 {
            let mut sum = grid[i][j];
            grid[i][j] = -1;

            let tos = DIRS
                .iter()
                .filter_map(|(di, dj)| {
                    let to_i = (i as i32 + di) as usize;
                    let to_j = (j as i32 + dj) as usize;
                    let _ = grid.get(to_i)?.get(to_j)?;
                    Some((to_i, to_j))
                })
                .collect::<Vec<_>>();

            for to in tos.into_iter() {
                if grid[to.0][to.1] > 0 {
                    sum += dfs(to, grid);
                }
            }

            sum
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] > 0 {
                    ans = ans.max(dfs((i, j), &mut grid));
                }
            }
        }

        ans
    }
}
