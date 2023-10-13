// https://leetcode.com/problems/min-cost-climbing-stairs

struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        dp[1] = 0;

        for i in 0..n {
            if i + 1 <= n {
                dp[i + 1] = dp[i + 1].min(dp[i] + cost[i]);
            }
            if i + 2 <= n {
                dp[i + 2] = dp[i + 2].min(dp[i] + cost[i]);
            }
        }

        dp[n]
    }
}
