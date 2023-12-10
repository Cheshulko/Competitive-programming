// https://leetcode.com/problems/transpose-matrix

struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix[0].len();
        matrix.into_iter().fold(vec![vec![]; m], |mut v, row| {
            row.into_iter().enumerate().for_each(|(j, x)| v[j].push(x));
            v
        })
    }
}
