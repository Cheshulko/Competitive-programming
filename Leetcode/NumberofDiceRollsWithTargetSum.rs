// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum

struct Solution {}

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let target = target as usize;

        let mut dp = vec![vec![0; target.max(k) + 1]; n + 1];
        for k_ in 1..=k {
            dp[1][k_] = 1;
        }

        for n_ in 2..=n {
            for t_ in 1..=target {
                for k_ in 1..=k.min(t_) {
                    dp[n_][t_] += dp[n_ - 1][t_ - k_];
                    dp[n_][t_] %= 1_000_000_000 + 7;
                }
            }
        }

        dp[n][target]
    }
}
