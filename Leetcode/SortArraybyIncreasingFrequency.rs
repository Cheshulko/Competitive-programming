// https://leetcode.com/problems/sort-array-by-increasing-frequency

use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let cnt = nums.iter().fold(vec![0; 201], |mut v, x| {
            v[(x + 100) as usize] += 1;
            v
        });

        let mut nums = nums
            .into_iter()
            .map(|x| (cnt[(x + 100) as usize], Reverse(x)))
            .collect::<Vec<_>>();
        nums.sort_unstable();

        nums.into_iter().map(|x| x.1 .0).collect::<Vec<_>>()
    }
}
