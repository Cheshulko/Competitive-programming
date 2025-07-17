// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii

struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut ans = 0;
        let mut dp = vec![vec![0; (k + 1) as usize]; n];
        for i in 0..n {
            for j in 0..i {
                let r = ((nums[j] + nums[i]) % k) as usize;

                dp[i][r] = dp[i][r].max(dp[j][r] + 1);
                ans = ans.max(dp[i][r]);
            }
        }

        ans + 1
    }
}
