// https://leetcode.com/problems/avoid-flood-in-the-city

struct Solution {}

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        use std::collections::{BTreeSet, HashMap};

        let mut free_days = BTreeSet::new();
        let mut state = HashMap::new();
        let mut ans = vec![1; rains.len()];
        for (i, r) in rains.into_iter().enumerate() {
            if r > 0 {
                ans[i] = -1;

                if let Some(&p) = state.get(&r) {
                    if let Some(&d) = free_days.range(p..).next() {
                        free_days.remove(&d);
                        ans[d] = r;
                    } else {
                        return vec![];
                    }
                }

                state.insert(r, i);
            } else {
                free_days.insert(i);
            }
        }

        ans
    }
}
