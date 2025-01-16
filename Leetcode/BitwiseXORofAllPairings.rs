// https://leetcode.com/problems/bitwise-xor-of-all-pairings

struct Solution {}

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let odd2 = nums2.len() & 1 == 1;
        let odd1 = nums1.len() & 1 == 1;

        let mut ans = 0;
        if odd2 {
            for x in nums1.into_iter() {
                ans ^= x;
            }
        }
        if odd1 {
            for x in nums2.into_iter() {
                ans ^= x;
            }
        }

        ans
    }
}
