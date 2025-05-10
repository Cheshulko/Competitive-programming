// https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros

struct Solution {}

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let nums1 = nums1.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let nums2 = nums2.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let cnt1 = nums1.iter().filter(|&x| *x == 0).count() as i64;
        let cnt2 = nums2.iter().filter(|&x| *x == 0).count() as i64;

        let s1 = nums1.iter().sum::<i64>() + cnt1;
        let s2 = nums2.iter().sum::<i64>() + cnt2;

        if s1 > s2 && cnt2 == 0 {
            return -1;
        }

        if s1 < s2 && cnt1 == 0 {
            return -1;
        }

        s1.max(s2)
    }
}
