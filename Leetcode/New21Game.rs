// https://leetcode.com/problems/new-21-game

struct Solution {}

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let (n, k, max_pts) = (n as usize, k as usize, max_pts as usize);

        if k == 0 {
            return 1.;
        }

        let mut dp = vec![0.; n + 1];
        dp[0] = 1.;

        let mut s = dp[0];
        for i in 1..=n.min(k + max_pts - 1) {
            dp[i] = s / (max_pts as f64);

            if i < k {
                s += dp[i];
            }

            if i >= max_pts {
                s -= dp[i - max_pts];
            }
        }

        let mut ans = 0.;
        for i in k..=n {
            ans += dp[i];
        }

        ans
    }
}
