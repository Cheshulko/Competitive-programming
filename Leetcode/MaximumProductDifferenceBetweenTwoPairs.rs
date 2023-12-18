// https://leetcode.com/problems/maximum-product-difference-between-two-pairs

struct Solution {}

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        nums[n - 1] * nums[n - 2] - nums[0] * nums[1]
    }
}
