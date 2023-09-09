// https://leetcode.com/problems/combination-sum-iv

struct Solution {}

impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for cur in 1..=target {
            for v in &nums {
                let p = (cur as i32) - v;
                if p >= 0 {
                    dp[cur] += dp[p as usize];
                }
            }
        }

        dp[target]
    }
}
