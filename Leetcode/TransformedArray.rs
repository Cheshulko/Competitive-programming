// https://leetcode.com/problems/transformed-array

struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering;

        let n = nums.len();

        nums.iter()
            .enumerate()
            .map(|(i, &x)| match x.cmp(&0) {
                Ordering::Equal => nums[i],
                _ => nums[(100 * n + i + nums[i] as usize) % n],
            })
            .collect()
    }
}
