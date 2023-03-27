// https://leetcode.com/problems/unique-paths-iii

use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn can((i, j): (i32, i32), n: i32, m: i32) -> bool {
        i >= 0 && i < n && j >= 0 && j < m
    }
    fn dfs(
        cnt: &mut usize,
        need: &usize,
        h: &mut HashSet<(i32, i32)>,
        ans: &mut usize,
        cur: (i32, i32),
        grid: &Vec<Vec<i32>>,
        last: (i32, i32),
    ) {
        if cur == last && *cnt == *need {
            *ans += 1;
            return;
        }

        h.insert(cur);
        for (di, dj) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let toi = cur.0 + di;
            let toj = cur.1 + dj;
            if Solution::can((toi, toj), grid.len() as i32, grid[0].len() as i32)
                && !h.contains(&(toi, toj))
            {
                if grid[toi as usize][toj as usize] != -1 {
                    *cnt += 1;
                    Solution::dfs(cnt, need, h, ans, (toi, toj), grid, last);
                    *cnt -= 1;
                }
            }
        }
        h.remove(&cur);
    }

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut h: HashSet<(i32, i32)> = HashSet::new();

        let mut si = 0;
        let mut sj = 0;

        let mut ei = 0;
        let mut ej = 0;

        let mut need = 0;

        for (i, x) in grid.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                if y == &1 {
                    si = i;
                    sj = j;
                    need += 1;
                } else if y == &2 {
                    ei = i;
                    ej = j;
                    need += 1;
                } else if y == &0 {
                    need += 1;
                }
            }
        }

        let mut cnt: usize = 1;
        let mut ans: usize = 0;

        Solution::dfs(
            &mut cnt,
            &need,
            &mut h,
            &mut ans,
            (si as i32, sj as i32),
            &grid,
            (ei as i32, ej as i32),
        );

        ans as i32
    }
}
