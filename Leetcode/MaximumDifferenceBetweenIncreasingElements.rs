// https://leetcode.com/problems/maximum-difference-between-increasing-elements

struct Solution {}

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut d = -1;
        for i in 0..n {
            for j in i + 1..n {
                if nums[j] > nums[i] {
                    d = d.max(nums[j] - nums[i]);
                }
            }
        }

        d
    }
}
