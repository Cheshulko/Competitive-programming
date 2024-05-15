// https://leetcode.com/problems/find-the-safest-path-in-a-grid

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    fn update((i, j): (usize, usize), dp: &mut Vec<Vec<i32>>) {
        let mut q = VecDeque::<(usize, usize, i32)>::new();
        q.push_back((i, j, 0));

        while let Some((i, j, w)) = q.pop_front() {
            if dp[i][j] > w {
                dp[i][j] = w;

                let i_ = i as i32;
                let j_ = j as i32;

                for (to_w, to_i, to_j) in
                    [(0, 1), (-1, 0), (0, -1), (1, 0)]
                        .into_iter()
                        .filter_map(|(di, dj)| {
                            let to_i = (i_ + di) as usize;
                            let to_j = (j_ + dj) as usize;
                            let to_w = *dp.get(to_i)?.get(to_j)?;

                            Some((to_w, to_i, to_j))
                        })
                {
                    if to_w > w + 1 {
                        q.push_back((to_i, to_j, w + 1));
                    }
                }
            }
        }
    }

    fn dfs(
        (i, j): (usize, usize),
        val: i32,
        dp: &Vec<Vec<i32>>,
        used: &mut Vec<Vec<bool>>,
    ) -> bool {
        used[i][j] = true;

        if dp[i][j] < val {
            return false;
        }

        if i == dp.len() - 1 && j == dp[0].len() - 1 {
            return true;
        }

        let i_ = i as i32;
        let j_ = j as i32;

        let mut can = false;
        for (to_i, to_j) in [(0, 1), (-1, 0), (0, -1), (1, 0)]
            .into_iter()
            .filter_map(|(di, dj)| {
                let to_i = (i_ + di) as usize;
                let to_j = (j_ + dj) as usize;
                let w = *dp.get(to_i)?.get(to_j)?;

                (w >= val).then_some((to_i, to_j))
            })
        {
            if !used[to_i][to_j] {
                can |= Solution::dfs((to_i, to_j), val, dp, used);
            }
        }

        can
    }

    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut dp = vec![vec![i32::MAX; m]; n];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    Solution::update((i, j), &mut dp);
                }
            }
        }

        let mut l = 0;
        let mut r = (n * m) as i32;

        while r - l > 1 {
            let mid = (r + l) / 2;

            let mut used = vec![vec![false; m]; n];
            let can = Solution::dfs((0, 0), mid, &dp, &mut used);

            if can {
                l = mid;
            } else {
                r = mid;
            }
        }

        l
    }
}
