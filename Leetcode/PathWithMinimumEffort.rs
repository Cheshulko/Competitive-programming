// https://leetcode.com/problems/path-with-minimum-effort

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let n = heights.len();
        let m = heights[0].len();
        let mut dp = vec![vec![i32::MAX; m]; n];

        dp[0][0] = 0;

        let mut bh = BinaryHeap::<Reverse<(i32, usize, usize)>>::new();
        bh.push(Reverse((0, 0, 0)));

        while let Some(Reverse((_, i, j))) = bh.pop() {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(di, dj)| {
                    let to_ = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                    let to = *heights.get(to_.0)?.get(to_.1)?;

                    let from: i32 = heights[i][j];
                    let d = dp[i][j].max((from - to).abs());
                    if dp[to_.0][to_.1] > d {
                        Some((d, to_.0, to_.1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(d, i, j)| {
                    dp[i][j] = d;
                    bh.push(Reverse((d, i, j)));
                })
        }

        dp[n - 1][m - 1]
    }
}
