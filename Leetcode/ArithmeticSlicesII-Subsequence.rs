// https://leetcode.com/problems/arithmetic-slices-ii-subsequence

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let n = nums.len();
        let mut hm = HashMap::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let d = nums[j] - nums[i];
                let cnt = *hm.get(&(i, d)).unwrap_or(&0);
                *hm.entry((j, d)).or_insert(0) += cnt + 1;
            }
        }

        let n = n as i64;
        let ans = hm.values().sum::<i64>() - n * (n - 1) / 2;

        ans as i32
    }
}
