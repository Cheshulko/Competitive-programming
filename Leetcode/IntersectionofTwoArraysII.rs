// https://leetcode.com/problems/intersection-of-two-arrays-ii

struct Solution {}

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let n = nums1.len();
        let m = nums2.len();

        let mut ans = vec![];
        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            while i < n && j < m && nums1[i] < nums2[j] {
                i += 1;
            }
            while i < n && j < m && nums2[j] < nums1[i] {
                j += 1;
            }
            while i < n && j < m && nums1[i] == nums2[j] {
                ans.push(nums1[i]);
                i += 1;
                j += 1;
            }
        }

        ans
    }
}
