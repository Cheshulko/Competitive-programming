// https://leetcode.com/problems/maximum-matrix-sum

struct Solution {}

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let n = matrix.len();

        let mut cnt = 0;
        let mut min = i64::MAX;
        let mut sum = 0;

        for i in 0..n {
            for j in 0..n {
                if matrix[i][j] < 0 {
                    cnt += 1;
                }
                let x = matrix[i][j].abs() as i64;
                min = min.min(x);
                sum += x;
            }
        }

        if cnt & 1 == 1 {
            sum - 2 * min
        } else {
            sum
        }
    }
}
