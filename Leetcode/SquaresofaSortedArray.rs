// https://leetcode.com/problems/squares-of-a-sorted-array

struct Solution {}

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        nums = nums.into_iter().map(|x| x * x).collect::<Vec<_>>();
        nums.sort();
        nums
    }
}
