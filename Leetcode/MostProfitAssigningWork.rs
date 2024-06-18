// https://leetcode.com/problems/most-profit-assigning-work

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        worker.sort_unstable();

        let mut dp = difficulty
            .into_iter()
            .zip(profit.into_iter())
            .collect::<Vec<_>>();

        dp.sort_unstable();

        let mut bh = BinaryHeap::<i32>::new();
        let mut ans = 0;
        let mut j = 0;
        for w in worker.into_iter() {
            while j < dp.len() && dp[j].0 <= w {
                bh.push(dp[j].1);
                j += 1;
            }
            ans += bh.peek().unwrap_or(&0);
        }

        ans
    }
}
