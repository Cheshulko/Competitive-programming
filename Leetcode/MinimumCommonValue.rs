// https://leetcode.com/problems/minimum-common-value

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();

        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            match nums1[i].cmp(&nums2[j]) {
                Ordering::Less => i += 1,
                Ordering::Greater => j += 1,
                Ordering::Equal => return nums1[i],
            }
        }

        -1
    }
}
