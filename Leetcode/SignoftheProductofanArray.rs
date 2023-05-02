// https://leetcode.com/problems/sign-of-the-product-of-an-array

struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().map(|x| x.signum()).product()
    }
}
