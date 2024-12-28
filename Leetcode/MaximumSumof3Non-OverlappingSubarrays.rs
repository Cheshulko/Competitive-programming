// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays

struct Solution {}

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        let mut dp = vec![vec![(i64::MIN, vec![]); 4]; n + 1];
        for i in 0..=n {
            dp[i][0] = (0, vec![]);
        }

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + nums[i] as i64;
        }

        for i in k..=n {
            for len in 1..=3 {
                dp[i][len] = dp[i - 1][len].clone();

                if dp[i - k][len - 1].0 != i64::MIN
                    && dp[i][len].0 < dp[i - k][len - 1].0 + pref[i] - pref[i - k]
                {
                    let mut pos = dp[i - k][len - 1].1.clone();
                    pos.push((i - k) as i32);
                    dp[i][len] = (dp[i - k][len - 1].0 + pref[i] - pref[i - k], pos);
                }
            }
        }

        dp[n][3].1.clone()
    }
}
