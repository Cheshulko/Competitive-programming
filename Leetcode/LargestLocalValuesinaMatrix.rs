// https://leetcode.com/problems/largest-local-values-in-a-matrix

struct Solution {}

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();

        let mut ans = vec![vec![0; m - 2]; n - 2];

        for i in 1..(n - 1) {
            for j in 1..(m - 1) {
                let mut x = 0;

                for ii in (i - 1)..=(i + 1) {
                    for jj in (j - 1)..=(j + 1) {
                        x = x.max(grid[ii][jj]);
                    }
                }

                ans[i - 1][j - 1] = x;
            }
        }

        ans
    }
}
