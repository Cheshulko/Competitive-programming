// https://leetcode.com/problems/find-triangular-sum-of-an-array

struct Solution {}

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() != 1 {
            let mut nnums = vec![];
            for i in 0..nums.len() - 1 {
                nnums.push((nums[i] + nums[i + 1]) % 10);
            }

            nums = nnums;
        }

        nums[0]
    }
}
