// https://leetcode.com/problems/matrix-similarity-after-cyclic-shifts

struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat.len();
        let m = mat[0].len();
        let mi = m as i32;

        for i in 0..n {
            let dir = (i as i32 & 1) * 2 - 1;

            for j in 0..m {
                if mat[i][j] != mat[i][(j as i32 + k * dir).rem_euclid(mi) as usize] {
                    return false;
                }
            }
        }

        true
    }
}
