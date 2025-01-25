// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();

        let mut nums_sorted = nums
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<_>>();
        nums_sorted.sort_unstable();

        let mut ans = vec![0; n];

        let mut j = 0;
        let mut start_j = 0;
        while j < n {
            let mut group_indxs = BTreeSet::new();

            let mut cur = nums_sorted[j].0;
            while j < n && nums_sorted[j].0 - cur <= limit {
                group_indxs.insert(nums_sorted[j].1);
                cur = nums_sorted[j].0;
                j += 1;
            }

            for ind in group_indxs.into_iter() {
                ans[ind] = nums_sorted[start_j].0;
                start_j += 1;
            }
        }

        ans
    }
}
