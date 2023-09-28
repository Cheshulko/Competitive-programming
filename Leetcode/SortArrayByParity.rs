// https://leetcode.com/problems/sort-array-by-parity

struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by(|a, b| (a % 2).cmp(&(b % 2)));
        nums
    }
}
