// https://leetcode.com/problems/special-positions-in-a-binary-matrix

struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut rows = vec![];
        let mut cols = vec![];

        for i in 0..n {
            rows.push((0..m).map(|j| mat[i][j]).sum::<i32>())
        }

        for j in 0..m {
            cols.push((0..n).map(|i| mat[i][j]).sum::<i32>())
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans += (rows[i] == 1 && cols[j] == 1 && mat[i][j] == 1) as i32
            }
        }

        ans
    }
}
