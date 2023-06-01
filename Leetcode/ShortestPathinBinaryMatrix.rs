// https://leetcode.com/problems/shortest-path-in-binary-matrix

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;

        let mut v = VecDeque::<(i32, i32, i32)>::new();
        let mut used = vec![vec![false; 101]; 101];

        if grid[0][0] == 0 {
            v.push_back((0, 0, 1));
            used[0][0] = true;
        }

        let mut ans = None;

        while let Some((i, j, dist)) = v.pop_front() {
            if i == n - 1 && j == n - 1 {
                ans = Some((i, j, dist));
                break;
            }

            for (di, dj) in &[
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, -1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ] {
                let to_i = i + di;
                let to_j = j + dj;
                if to_i >= 0
                    && to_i < n
                    && to_j >= 0
                    && to_j < n
                    && !used[to_i as usize][to_j as usize]
                    && grid[to_i as usize][to_j as usize] == 0
                {
                    v.push_back((to_i, to_j, dist + 1));
                    used[to_i as usize][to_j as usize] = true;
                }
            }
        }

        if let Some(ans) = ans {
            ans.2
        } else {
            -1
        }
    }
}
