// https://leetcode.com/problems/count-submatrices-with-equal-frequency-of-x-and-y

struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut X = vec![vec![0; m + 1]; n + 1];
        let mut Y = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                X[i + 1][j + 1] = (grid[i][j] == 'X') as i64 + X[i][j + 1] + X[i + 1][j] - X[i][j];
                Y[i + 1][j + 1] = (grid[i][j] == 'Y') as i64 + Y[i][j + 1] + Y[i + 1][j] - Y[i][j];
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans += (X[i + 1][j + 1] == Y[i + 1][j + 1] && X[i + 1][j + 1] > 0) as i32;
            }
        }

        ans
    }
}
