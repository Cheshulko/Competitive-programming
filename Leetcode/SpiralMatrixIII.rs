// https://leetcode.com/problems/spiral-matrix-iii

struct Solution {}

impl Solution {
    pub fn spiral_matrix_iii(
        rows: i32,
        cols: i32,
        mut r_start: i32,
        mut c_start: i32,
    ) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut ind = 0;
        let mut k = 1;
        for _ in 0..2 * rows.max(cols) {
            for _ in 0..2 {
                for _ in 0..k {
                    if r_start >= 0 && r_start < rows && c_start >= 0 && c_start < cols {
                        ans.push(vec![r_start, c_start]);
                    }
                    c_start += dirs[ind].0;
                    r_start += dirs[ind].1;
                }
                ind += 1;
                ind %= dirs.len();
            }
            k += 1;
        }

        ans
    }
}
