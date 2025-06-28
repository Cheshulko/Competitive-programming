// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum

struct Solution {}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_by_key(|&(_, x)| x);

        let mut nums = nums.into_iter().rev().take(k as usize).collect::<Vec<_>>();
        nums.sort_by_key(|&(i, _)| i);

        nums.into_iter().map(|(_, x)| x).collect()
    }
}
