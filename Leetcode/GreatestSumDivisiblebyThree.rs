// https://leetcode.com/problems/greatest-sum-divisible-by-three

struct Solution {}

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dp = vec![[0; 3]; n + 1];
        dp[0][1] = i32::MIN;
        dp[0][2] = i32::MIN;
        for (i, x) in nums.into_iter().enumerate() {
            let i = i + 1;

            for r in 0..3 {
                dp[i][r] = dp[i - 1][r];
            }

            let y = x as usize % 3;
            for r in 0..3 {
                dp[i][(r + y) % 3] = dp[i][(r + y) % 3].max(dp[i - 1][r] + x);
            }
        }

        dp[n][0]
    }
}
