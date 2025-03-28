// https://leetcode.com/problems/maximum-number-of-points-from-grid-queries

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

        let n = grid.len();
        let m = grid[0].len();

        let mut ans = vec![];
        let mut points = vec![];

        let mut cnt = 0;
        let mut cur = 0;
        let mut seen = vec![vec![false; m]; n];

        let mut bh = BinaryHeap::new();
        bh.push((Reverse(grid[0][0]), (0, 0)));
        seen[0][0] = true;

        while let Some((Reverse(v), (i, j))) = bh.pop() {
            if v >= cur {
                points.push(cur);
                ans.push(cnt);

                cur = v + 1;
            }

            cnt += 1;

            for (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (i as i32 + di) as usize;
                let to_j = (j as i32 + dj) as usize;
                let _ = grid.get(to_i)?.get(to_j)?;

                Some((to_i, to_j))
            }) {
                if seen[to_i][to_j] {
                    continue;
                }

                seen[to_i][to_j] = true;
                bh.push((Reverse(grid[to_i][to_j]), (to_i, to_j)));
            }
        }

        points.push(cur);
        ans.push(cnt);

        queries
            .into_iter()
            .map(|q| {
                let p = points.partition_point(|&p| p <= q);
                ans[p - 1]
            })
            .collect()
    }
}
