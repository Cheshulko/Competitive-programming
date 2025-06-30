// https://leetcode.com/problems/longest-harmonious-subsequence

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let cnt = nums
            .into_iter()
            .fold(BTreeMap::<i32, i32>::new(), |mut bt, x| {
                *bt.entry(x).or_default() += 1;
                bt
            });

        let mut ans = 0;
        for ((n1, cnt1), (n2, cnt2)) in cnt.iter().zip(cnt.iter().skip(1)) {
            if n2 - n1 != 1 {
                continue;
            }

            ans = ans.max(cnt1 + cnt2);
        }

        ans
    }
}
