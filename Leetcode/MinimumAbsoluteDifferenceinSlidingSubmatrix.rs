// https://leetcode.com/problems/minimum-absolute-difference-in-sliding-submatrix

struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::BTreeSet;

        fn get(grid: &Vec<Vec<i32>>, k: usize, i: usize, j: usize) -> i32 {
            let mut s = BTreeSet::new();
            for ii in i..i + k {
                for jj in j..j + k {
                    s.insert(grid[ii][jj]);
                }
            }

            if s.len() <= 1 {
                return 0;
            }

            s.iter()
                .zip(s.iter().skip(1))
                .map(|(&a, &b)| b.abs_diff(a))
                .min()
                .unwrap() as i32
        }

        let k = k as usize;
        let n = grid.len();
        let m = grid[0].len();

        let mut ans = vec![vec![0; m - k + 1]; n - k + 1];
        for i in 0..n - k + 1 {
            for j in 0..m - k + 1 {
                ans[i][j] = get(&grid, k, i, j);
            }
        }

        ans
    }
}
