// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps

struct Solution {}

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let m = 1_000_000_000 + 7;
        let arr_len = arr_len.min(steps /* /2 */) as usize;
        let steps = steps as usize;

        let mut dp = vec![vec![0; arr_len]; steps + 1];
        dp[steps][0] = 1;

        for st in (1..=steps).rev() {
            for pos in 0..arr_len {
                dp[st - 1][pos] = (dp[st - 1][pos] + dp[st][pos]) % m;

                if pos + 1 < arr_len {
                    dp[st - 1][pos + 1] = (dp[st - 1][pos + 1] + dp[st][pos]) % m;
                }

                if pos > 0 {
                    dp[st - 1][pos - 1] = (dp[st - 1][pos - 1] + dp[st][pos]) % m;
                }
            }
        }

        dp[0][0]
    }
}
