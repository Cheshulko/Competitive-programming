// https://leetcode.com/problems/house-robber

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];

        let mut ans = 0;
        for cur in 0..n {
            let num = nums[cur];
            dp[cur] = dp[cur].max(num);
            ans = ans.max(dp[cur]);

            for to in (cur + 2)..n {
                dp[to] = dp[to].max(nums[to] + dp[cur]);
            }
        }

        ans
    }
}
