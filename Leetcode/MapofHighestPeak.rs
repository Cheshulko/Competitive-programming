// https://leetcode.com/problems/map-of-highest-peak

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

        let n = is_water.len();
        let m = is_water[0].len();

        let mut used = vec![vec![false; m]; n];
        let mut height = vec![vec![0; m]; n];
        let mut q = VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if is_water[i][j] == 1 {
                    used[i][j] = true;
                    q.push_back((i, j));
                }
            }
        }

        if q.is_empty() {
            used[0][0] = true;
            q.push_back((0, 0));
        }

        while let Some((i, j)) = q.pop_front() {
            let cur_height = height[i][j];

            for (to_i, to_j) in DIRS.iter().filter_map(|(di, dj)| {
                let to_i = (i as i32 + di) as usize;
                let to_j = (j as i32 + dj) as usize;
                let _ = is_water.get(to_i)?.get(to_j)?;

                Some((to_i, to_j))
            }) {
                if !used[to_i][to_j] {
                    height[to_i][to_j] = cur_height + 1;
                    used[to_i][to_j] = true;
                    q.push_back((to_i, to_j));
                }
            }
        }

        height
    }
}
