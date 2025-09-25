// https://leetcode.com/problems/triangle

struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();

        let mut dp = vec![i32::MAX; n + 1];
        let mut ndp = vec![i32::MAX; n + 1];
        dp[0] = triangle[0][0];

        for i in 1..n {
            ndp.fill(i32::MAX);
            for j in 0..triangle[i].len() {
                if j < triangle[i].len() - 1 {
                    ndp[j] = ndp[j].min(dp[j] + triangle[i][j]);
                }
                if j > 0 {
                    ndp[j] = ndp[j].min(dp[j - 1] + triangle[i][j]);
                }
            }

            std::mem::swap(&mut dp, &mut ndp);
        }

        (0..triangle.last().unwrap().len())
            .map(|i| dp[i])
            .min()
            .unwrap()
    }
}
