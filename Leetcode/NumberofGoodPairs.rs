// https://leetcode.com/problems/number-of-good-pairs

struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .flat_map(|a| nums.iter().enumerate().map(move |b| (a.clone(), b)))
            .filter(|(a, b)| a.0 < b.0 && a.1 == b.1)
            .count() as i32
    }
}
