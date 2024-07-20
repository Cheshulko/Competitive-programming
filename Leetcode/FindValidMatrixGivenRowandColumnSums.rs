// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums

struct Solution {}

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let n = row_sum.len();
        let m = col_sum.len();

        let mut a = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                a[i][j] = row_sum[i].min(col_sum[j]);
                row_sum[i] -= a[i][j];
                col_sum[j] -= a[i][j];
            }
        }

        a
    }
}
