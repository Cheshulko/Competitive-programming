// https://leetcode.com/problems/longest-ideal-subsequence

struct Solution {}

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let n = s.len();

        let mut dp = vec![0; 26];
        let mut ans = 0;

        for i in 1..=n {
            let c = (s[i - 1] - b'a') as i32;
            let c_ = c as usize;

            dp[c_] = dp[c_].max(dp[c_] + 1).max(1);

            for p in (c - k).max(0)..c {
                dp[c_] = dp[c_].max(dp[p as usize] + 1);
            }

            for p in (c + 1)..=(c + k).min(25) {
                dp[c_] = dp[c_].max(dp[p as usize] + 1);
            }

            ans = ans.max(dp[c_]);
        }

        ans
    }
}
