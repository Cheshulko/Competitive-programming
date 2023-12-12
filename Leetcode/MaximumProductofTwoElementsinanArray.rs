// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array

struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .flat_map(|(i, a)| nums.iter().skip(i + 1).map(move |b| (a - 1) * (b - 1)))
            .max()
            .unwrap()
    }
}
