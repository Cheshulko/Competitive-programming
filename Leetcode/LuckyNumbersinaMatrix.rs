// https://leetcode.com/problems/lucky-numbers-in-a-matrix

struct Solution {}

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let min_row = |r: usize, matrix: &Vec<Vec<i32>>| {
            let mut min = i32::MAX;
            for j in 0..matrix[0].len() {
                min = min.min(matrix[r][j]);
            }

            min
        };

        let max_col = |c: usize, matrix: &Vec<Vec<i32>>| {
            let mut max = i32::MIN;
            for i in 0..matrix.len() {
                max = max.max(matrix[i][c]);
            }

            max
        };

        let rows = (0..matrix.len())
            .map(|r| min_row(r, &matrix))
            .collect::<Vec<_>>();
        let cols = (0..matrix[0].len())
            .map(|c| max_col(c, &matrix))
            .collect::<Vec<_>>();

        let mut ans = vec![];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let x = matrix[i][j];
                if x == rows[i] && x == cols[j] {
                    ans.push(x);
                }
            }
        }

        ans
    }
}
