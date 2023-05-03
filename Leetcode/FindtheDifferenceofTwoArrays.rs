// https://leetcode.com/problems/find-the-difference-of-two-arrays

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        vec![
            nums1
                .clone()
                .into_iter()
                .filter(|x| !nums2.contains(x))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            nums2
                .clone()
                .into_iter()
                .filter(|x| !nums1.contains(x))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
        ]
    }
}
