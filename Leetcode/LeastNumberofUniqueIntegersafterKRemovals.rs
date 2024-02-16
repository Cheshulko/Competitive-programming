// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let cnt = arr.into_iter().fold(HashMap::new(), |mut hm, x| {
            *hm.entry(x).or_insert(0) += 1;
            hm
        });

        let mut cnt = cnt.values().collect::<Vec<_>>();
        cnt.sort();

        let mut s = 0;
        let mut ans = 0;
        while ans < cnt.len() && s + cnt[ans] <= k {
            s += cnt[ans];
            ans += 1;
        }

        (cnt.len() - ans) as i32
    }
}
