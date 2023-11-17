// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array

struct Solution {}

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let half_n = nums.len() / 2;
        nums.sort();
        nums.iter()
            .take(half_n)
            .zip(nums.iter().rev().take(half_n))
            .map(|(a, b)| a + b)
            .max()
            .unwrap()
    }
}
