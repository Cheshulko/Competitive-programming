// https://leetcode.com/problems/intersection-of-two-arrays

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1 = nums1.into_iter().collect::<HashSet<_>>();
        let nums2 = nums2.into_iter().collect::<HashSet<_>>();

        nums1
            .intersection(&nums2)
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>()
    }
}
