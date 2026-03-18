// https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k

struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as i64;
        let n = grid.len();
        let m = grid[0].len();

        let mut sum = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                sum[i + 1][j + 1] = grid[i][j] as i64 + sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans += (sum[i + 1][j + 1] <= k) as i32;
            }
        }

        ans
    }
}
