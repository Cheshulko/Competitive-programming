// https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference

struct Solution {}

impl Solution {
    pub fn longest_subsequence(mut arr: Vec<i32>, difference: i32) -> i32 {
        let mx = 1_000_00;
        let mut dp = vec![0; 2 * mx + 1];

        for a in arr.iter_mut() {
            *a += mx as i32;
        }

        let mut ans = 1;

        for a in arr.into_iter() {
            let prev = a - difference;
            if prev > 0 {
                dp[a as usize] = dp[a as usize].max(dp[prev as usize] + 1).max(1);
                ans = ans.max(dp[a as usize]);
            }
        }

        ans
    }
}
