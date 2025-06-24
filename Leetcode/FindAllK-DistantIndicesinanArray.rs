// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let n = nums.len() as i32;

        nums.iter()
            .enumerate()
            .filter_map(|(i, &x)| (x == key).then_some(i as i32))
            .fold(BTreeSet::new(), |mut hs, i| {
                hs.extend(i - k..=i + k);
                hs
            })
            .into_iter()
            .filter(|&j| j >= 0 && j < n)
            .collect()
    }
}
