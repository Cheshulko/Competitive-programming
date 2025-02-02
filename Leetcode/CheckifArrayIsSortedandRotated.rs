// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated

struct Solution {}

impl Solution {
    pub fn check(mut nums: Vec<i32>) -> bool {
        let mut nums_sorted = nums.clone();
        nums_sorted.sort_unstable();

        nums.extend(nums.clone());

        for w in nums.windows(nums_sorted.len()) {
            if w == nums_sorted {
                return true;
            }
        }

        false
    }
}
