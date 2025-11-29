// https://leetcode.com/problems/minimum-operations-to-make-array-sum-divisible-by-k

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}
