// https://leetcode.com/problems/01-matrix

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut q = VecDeque::<(usize, usize)>::new();
        let mut ans = vec![vec![i32::MAX; m]; n];

        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 0 {
                    ans[i][j] = 0;
                    q.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = q.pop_front() {
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter_map(|(di, dj)| {
                    let to_i = (i as i32 + *di) as usize;
                    let to_j = (j as i32 + *dj) as usize;
                    if mat.get(to_i)?.get(to_j)? == &1 {
                        Some((to_i, to_j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for (i1, j1) in dirs.into_iter() {
                if ans[i1][j1] > ans[i][j] + 1 {
                    ans[i1][j1] = ans[i][j] + 1;
                    q.push_back((i1, j1));
                }
            }
        }

        ans
    }
}
