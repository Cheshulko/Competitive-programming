// https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/description

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    const DIRS: &'static [(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn solve((i, j): (usize, usize), ones: usize, grid: &Vec<Vec<i32>>) -> usize {
        let mut ans = usize::MAX;

        let n = grid.len();
        let m = grid[0].len();

        let mut used = vec![vec![false; m]; n];

        let mut q = VecDeque::<(usize, usize)>::new();
        q.push_back((i, j));
        used[i][j] = true;

        let mut processed = 0;

        while let Some((cur_i, cur_j)) = q.pop_front() {
            processed += 1;

            let dirs = Solution::DIRS
                .iter()
                .filter_map(|(di, dj)| {
                    let to_i = cur_i as i32 + di;
                    let to_j = cur_j as i32 + dj;

                    (to_i >= 0
                        && to_i < n as i32
                        && to_j >= 0
                        && to_j < m as i32
                        && grid[to_i as usize][to_j as usize] == 1
                        && !used[to_i as usize][to_j as usize])
                        .then_some((to_i as usize, to_j as usize))
                })
                .collect::<Vec<_>>();

            for (to_i, to_j) in dirs.into_iter() {
                used[to_i][to_j] = true;

                q.push_back((to_i, to_j));
            }

            if processed + q.len() < ones {
                ans = ans.min(q.len());
            }
        }

        ans
    }

    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut ones = 0;
        for i in 0..n {
            for j in 0..m {
                ones += grid[i][j];
            }
        }

        let mut ans = ones as usize;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] > 0 {
                    ans = ans.min(Solution::solve((i, j), ones as usize, &grid))
                }
            }
        }

        ans as i32
    }
}
