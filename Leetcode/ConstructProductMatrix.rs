// https://leetcode.com/problems/construct-product-matrix

struct Solution;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const M: i64 = 12345;

        let n = grid.len();
        let m = grid[0].len();

        let mut rows = vec![1; n + 1];
        for i in 0..n {
            for j in 0..m {
                rows[i] = rows[i] * grid[i][j] as i64 % M;
            }
        }

        let mut row_pref = vec![1; n + 1];
        for i in 0..n {
            row_pref[i + 1] = row_pref[i] * rows[i] % M;
        }

        let mut row_suf = vec![1; n + 1];
        for i in (0..n).rev() {
            row_suf[i] = row_suf[i + 1] * rows[i] % M;
        }

        let mut ans = vec![vec![1; m]; n];
        for i in 0..n {
            let mut pref = vec![1; m + 1];
            for j in 0..m {
                pref[j + 1] = pref[j] * grid[i][j] as i64 % M;
            }

            let mut suf = vec![1; m + 1];
            for j in (0..m).rev() {
                suf[j] = suf[j + 1] * grid[i][j] as i64 % M;
            }

            for j in 0..m {
                ans[i][j] = (row_pref[i] * row_suf[i + 1] * pref[j] * suf[j + 1] % M) as i32;
            }
        }

        ans
    }
}
