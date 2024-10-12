// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v[1], v[0], i))
            .collect::<Vec<_>>();

        intervals.sort_unstable();

        let mut groups = BTreeSet::<(i32, usize)>::new();
        for (end, st, i) in intervals.into_iter() {
            if let Some(mut int) = groups.range(..(st - 1, usize::MAX)).next_back().copied() {
                groups.remove(&int);
                int.0 = end;
                groups.insert(int);
            } else {
                groups.insert((end, i));
            }
        }

        groups.len() as i32
    }
}
