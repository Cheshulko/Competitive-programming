// https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three

struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|n| (n % 3).min(3 - n % 3)).sum()
    }
}
