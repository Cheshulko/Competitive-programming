// https://leetcode.com/problems/target-sum

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = HashMap::<i32, i32>::new();
        dp.insert(0, 1);

        for num in nums {
            let mut ndp = HashMap::<i32, i32>::new();
            for (&have_num, &cnt) in dp.iter() {
                *ndp.entry(have_num - num).or_default() += cnt;
                *ndp.entry(have_num + num).or_default() += cnt;
            }

            dp = ndp;
        }

        *dp.get(&target).unwrap_or(&0)
    }
}
