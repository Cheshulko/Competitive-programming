// https://leetcode.com/problems/contiguous-array

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pref_sum = 0;

        let mut cnt = HashMap::<i32, i32>::new();
        cnt.insert(0, -1);

        for (i, num) in nums.into_iter().enumerate() {
            let i = i as i32;
            pref_sum += if num == 1 { 1 } else { -1 };

            if let Some(j) = cnt.get(&pref_sum) {
                ans = ans.max(i - *j);
            } else {
                cnt.insert(pref_sum, i);
            }
        }

        ans as i32
    }
}
