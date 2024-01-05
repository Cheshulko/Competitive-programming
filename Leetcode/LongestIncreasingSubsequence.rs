// https://leetcode.com/problems/longest-increasing-subsequence

struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if dp[i] < dp[j] + 1 && nums[i] > nums[j] {
                    dp[i] = dp[j] + 1;
                }
            }
        }

        dp.into_iter().max().unwrap()
    }
}
