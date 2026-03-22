// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation

struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();

        let mut ok = [true; 4];
        for i in 0..n {
            for j in 0..n {
                ok[0] &= mat[i][j] == target[i][j];
                ok[1] &= mat[i][j] == target[n - 1 - j][i];
                ok[2] &= mat[i][j] == target[n - 1 - i][n - 1 - j];
                ok[3] &= mat[i][j] == target[j][n - 1 - i];
            }
        }

        ok.contains(&true)
    }
}
