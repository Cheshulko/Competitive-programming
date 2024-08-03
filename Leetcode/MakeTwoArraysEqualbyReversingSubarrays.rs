// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays

struct Solution {}

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();

        target == arr
    }
}
