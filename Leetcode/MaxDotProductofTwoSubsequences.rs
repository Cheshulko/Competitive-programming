// https://leetcode.com/problems/max-dot-product-of-two-subsequences

struct Solution {}

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut dp = vec![vec![0; m]; n];

        dp[0][0] = nums1[0] * nums2[0];
        let mut ans = dp[0][0];

        for i in 1..n {
            dp[i][0] = dp[i - 1][0].max(nums1[i] * nums2[0]);
            ans = ans.max(dp[i][0]);
        }

        for j in 1..m {
            dp[0][j] = dp[0][j - 1].max(nums1[0] * nums2[j]);
            ans = ans.max(dp[0][j]);
        }

        for i in 1..n {
            for j in 1..m {
                dp[i][j] = dp[i - 1][j]
                    .max(dp[i][j - 1])
                    .max(dp[i - 1][j - 1] + nums1[i] * nums2[j])
                    .max(nums1[i] * nums2[j]);
                ans = ans.max(dp[i][j]);
            }
        }

        ans
    }
}
