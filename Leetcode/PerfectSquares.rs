// https://leetcode.com/problems/perfect-squares

struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mx = (n as f32).sqrt().floor() as usize;
        let mut dp = vec![i32::MAX / 2; n + 1];

        for x in 1..=mx {
            dp[x * x] = 1;
        }

        for x in 1..=n {
            for i in 1..x {
                dp[x] = dp[x].min(dp[i] + dp[x - i]);
            }
        }

        dp[n]
    }
}
