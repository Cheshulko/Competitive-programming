// https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut bh = nums
            .into_iter()
            .map(|x| Reverse(x as i64))
            .collect::<BinaryHeap<_>>();

        let mut ans = 0;
        while let (Some(Reverse(x)), Some(Reverse(y))) = (bh.pop(), bh.pop()) {
            if x >= k {
                break;
            }

            ans += 1;
            bh.push(Reverse(x.min(y) * 2 + x.max(y)));
        }

        ans
    }
}
