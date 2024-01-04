// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(HashMap::new(), |mut hm, x| {
                *hm.entry(x).or_insert(0) += 1;
                hm
            })
            .into_iter()
            .map(|(_, value)| (value != 1).then_some(value / 3 + (value % 3 + 1) / 2))
            .fold(Some(0), |x, s| x.and(s).map(|y| x.unwrap() + y))
            .unwrap_or(-1)
    }
}
