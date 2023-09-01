// https://leetcode.com/problems/counting-bits

struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;

        let mut dp = vec![0; n + 1];
        let mut p = 1;
        let mut cur = 1;

        for i in 1..=n {
            if i == p {
                cur = i;
                dp[i] = 1;
                p <<= 1;
            } else {
                dp[i] = dp[cur] + dp[i - cur];
            }
        }

        dp
    }
}
