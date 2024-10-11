// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-chair

use std::cmp::Ordering;
use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let n = times.len();
        let target_friend = target_friend as usize;

        let mut times = times
            .into_iter()
            .enumerate()
            .flat_map(|(i, t)| [(t[0], 0, i), (t[1], 1, i)])
            .collect::<Vec<_>>();

        times.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => b.1.cmp(&a.1),
            x => x,
        });

        let mut free = (0..n).into_iter().collect::<BTreeSet<_>>();
        let mut ans = vec![0; n];
        for (_, t, i) in times.into_iter() {
            if t == 0 {
                let sit = *free.first().unwrap();
                free.remove(&sit);
                ans[i] = sit;
            } else {
                free.insert(ans[i]);
            }
        }

        ans[target_friend] as i32
    }
}
