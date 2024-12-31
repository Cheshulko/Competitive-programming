// https://leetcode.com/problems/minimum-cost-for-tickets

struct Solution {}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; 366];
        dp[0] = 0;

        let range_costs = [1, 7, 30]
            .into_iter()
            .zip(costs.into_iter())
            .collect::<Vec<_>>();

        let mut prev_days = vec![0];
        for day in days {
            let day_u = day as usize;

            for &(range, cost) in range_costs.iter() {
                if day >= range {
                    let p = prev_days.partition_point(|&x| x <= day - range);
                    dp[day_u] = dp[day_u].min(dp[prev_days[p - 1] as usize] + cost);
                } else {
                    dp[day_u] = dp[day_u].min(cost);
                }
            }

            prev_days.push(day);
        }

        dp[*prev_days.last().unwrap() as usize]
    }
}
