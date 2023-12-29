// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule

struct Solution {}

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;

        if n < d {
            return -1;
        }

        let mut dp = vec![vec![0; n + 1]; d + 1];

        for jobs in 1..=n {
            dp[1][jobs] = dp[1][jobs - 1].max(job_difficulty[jobs - 1]);
        }

        for days in 2..=d {
            for jobs in days..=n {
                dp[days][jobs] = i32::MAX;
                let mut mx = job_difficulty[jobs - 1];
                for jobs_for_prev_days in ((days - 1)..=(jobs - 1)).rev() {
                    mx = mx.max(job_difficulty[jobs_for_prev_days]);
                    dp[days][jobs] = dp[days][jobs].min(dp[days - 1][jobs_for_prev_days] + mx);
                }
            }
        }

        dp[d][n]
    }
}
