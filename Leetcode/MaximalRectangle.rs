// https://leetcode.com/problems/maximal-rectangle

struct Solution {}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut dp = vec![vec![0; m + 1]; n];
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == '1' {
                    dp[i][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i][j + 1] = 0;
                }

                ans = ans.max(dp[i][j + 1]);
            }
        }

        for i in 0..n {
            let mut cur = vec![i32::MAX; m + 1];
            for j in i..n {
                for k in 1..(m + 1) {
                    cur[k] = cur[k].min(dp[j][k]);
                    ans = ans.max(cur[k] * (j - i + 1) as i32);
                }
            }
        }

        ans
    }
}
