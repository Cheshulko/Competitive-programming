// https://leetcode.com/problems/missing-number

struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let sum = nums.iter().sum::<i64>();

        let n = nums.len() as i64;
        (n * (n + 1) / 2 - sum) as i32
    }
}
