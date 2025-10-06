// https://leetcode.com/problems/swim-in-rising-water

struct Solution {}

impl Solution {
    const DIRS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1), (1, 0)];

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let h = grid.len();
        let w = grid[0].len();

        let mut seen = vec![vec![false; w]; h];

        Solution::bfs(&grid, grid[0][0], &mut seen);
        if seen[h - 1][w - 1] {
            return grid[0][0];
        }

        let ma = grid
            .iter()
            .map(|row| row.iter().max().copied().unwrap_or(0))
            .max()
            .unwrap_or(0);

        let mut l = grid[0][0];
        let mut r = ma;

        while r - l > 1 {
            for i in 0..h {
                for j in 0..w {
                    seen[i][j] = false;
                }
            }
            let m = (r + l) >> 1;
            Solution::bfs(&grid, m, &mut seen);

            if seen[h - 1][w - 1] {
                r = m;
            } else {
                l = m;
            }
        }

        r
    }

    fn bfs(grid: &Vec<Vec<i32>>, t: i32, seen: &mut Vec<Vec<bool>>) {
        let mut q = vec![(0, 0)];
        while let Some((i, j)) = q.pop() {
            seen[i][j] = true;

            for (to_i, to_j) in Solution::DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (i as i32 + di) as usize;
                let to_j = (j as i32 + dj) as usize;
                let h = *grid.get(to_i)?.get(to_j)?;

                (h <= t).then_some((to_i, to_j))
            }) {
                if seen[to_i][to_j] {
                    continue;
                }

                seen[to_i][to_j] = true;
                q.push((to_i, to_j));
            }
        }
    }
}
