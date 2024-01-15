// https://leetcode.com/problems/find-players-with-zero-or-one-losses

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let losses = matches.into_iter().fold(HashMap::new(), |mut losses, m| {
            *losses.entry(m[1]).or_insert(0) += 1;
            losses.entry(m[0]).or_insert(0);
            losses
        });

        let losses_cnt = |cnt: i32| {
            losses
                .iter()
                .filter_map(|(k, v)| (v == &cnt).then_some(*k))
                .collect::<Vec<_>>()
        };

        let mut zeros = losses_cnt(0);
        let mut ones = losses_cnt(1);

        zeros.sort_unstable();
        ones.sort_unstable();

        vec![zeros, ones]
    }
}
