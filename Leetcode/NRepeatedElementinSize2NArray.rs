// https://leetcode.com/problems/n-repeated-element-in-size-2n-array

struct Solution {}

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() - 2 {
            if nums[i] == nums[i + 1] || nums[i] == nums[i + 2] {
                return nums[i];
            }
        }

        nums.last().copied().unwrap()
    }
}
