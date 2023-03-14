// https://leetcode.com/problems/reducing-dishes/description/

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction_mut = satisfaction;
        satisfaction_mut.sort();
        let len = satisfaction_mut.len();
        let mut dp = vec![vec![-1000000 * 555; len + 2]; len + 2];
        for i in 0..len + 2 {
            dp[i][0] = 0;
        }
        dp[0][1] = 1 * satisfaction_mut[0];
        for i in 1..len {
            for j in 1..i + 2 {
                for k in 0..i {
                    dp[i][j] =
                        std::cmp::max(dp[i][j], dp[k][j - 1] + (j as i32) * satisfaction_mut[i]);
                }
            }
        }
        let mut ans = 0;
        for i in 0..len {
            for j in 0..len + 1 {
                ans = std::cmp::max(ans, dp[i][j]);
            }
        }

        ans
    }
}
