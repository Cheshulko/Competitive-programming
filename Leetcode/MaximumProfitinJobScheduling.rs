// https://leetcode.com/problems/maximum-profit-in-job-scheduling

use std::cmp::Ordering;
use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut data = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .collect::<Vec<_>>();

        data.sort_by(|a, b| match a.0 .1.cmp(&b.0 .1) {
            Ordering::Equal => a.0 .0.cmp(&b.0 .0),
            x @ _ => x,
        });

        let mut ans = 0;
        let mut bt = BTreeMap::new();

        for ((st, en), pr) in data.into_iter() {
            let less_or_equal = bt.range(..=st).next_back();

            let best_ends_at_en = if let Some(less_or_equal) = less_or_equal {
                less_or_equal.1 + pr
            } else {
                pr
            };

            ans = ans.max(best_ends_at_en);
            bt.insert(en, best_ends_at_en.max(ans));
        }

        ans
    }
}
