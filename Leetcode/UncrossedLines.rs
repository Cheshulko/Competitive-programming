// https://leetcode.com/problems/uncrossed-lines

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (n, m) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; m]; n];

        dp[0][0] = (nums1[0] == nums2[0]) as i32;

        for j in 1..m {
            dp[0][j] = (nums1[0] == nums2[j]) as i32;
            dp[0][j] = dp[0][j].max(dp[0][j - 1]);
        }

        for i in 1..n {
            dp[i][0] = (nums1[i] == nums2[0]) as i32;
            dp[i][0] = dp[i][0].max(dp[i - 1][0]);
        }

        for i in 1..n {
            for j in 1..m {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
                    .max(dp[i - 1][j - 1] + (nums1[i] == nums2[j]) as i32);
            }
        }

        dp[n - 1][m - 1]
    }
}
