// https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon

struct Solution {}

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = energy.len();
        let mut dp = vec![i32::MIN; n];

        let mut ans = i32::MIN;
        for (i, e) in energy.into_iter().enumerate() {
            dp[i] = dp[i].max(e);
            if i >= k {
                dp[i] = dp[i].max(dp[i - k] + e);
            }

            if i + k >= n {
                ans = ans.max(dp[i]);
            }
        }

        ans
    }
}
