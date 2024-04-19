// https://leetcode.com/problems/number-of-islands

struct Solution {}

impl Solution {
    const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn dfs(grid: &mut Vec<Vec<char>>, cur: (usize, usize)) {
        grid[cur.0][cur.1] = 'x';

        let x = Solution::DIRS
            .iter()
            .filter_map(|(di, dj)| {
                let to_i = (cur.0 as i32 + di) as usize;
                let to_j = (cur.1 as i32 + dj) as usize;
                (grid.get(to_i)?.get(to_j)? == &'1').then_some((to_i, to_j))
            })
            .collect::<Vec<_>>();

        for (to_i, to_j) in x.into_iter() {
            if grid[to_i][to_j] != 'x' {
                Solution::dfs(grid, (to_i, to_j));
            }
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    ans += 1;
                    Solution::dfs(&mut grid, (i, j));
                }
            }
        }

        ans
    }
}
