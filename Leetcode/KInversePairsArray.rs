// https://leetcode.com/problems/k-inverse-pairs-array

struct Solution {}

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        const MOD: i64 = 1_000_000_000 + 7;

        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 1..=n {
            dp[i][0] = 1;
        }

        for i in 1..=n {
            for k_ in 1..=k {
                for dk in 0..=k_.min(i - 1) {
                    dp[i][k_] += dp[i - 1][k_ - dk];
                    dp[i][k_] %= MOD;
                }
            }
        }

        dp[n][k] as i32
    }
}
