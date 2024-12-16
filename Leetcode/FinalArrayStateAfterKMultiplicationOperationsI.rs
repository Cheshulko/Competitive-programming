// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut bh = nums
            .into_iter()
            .enumerate()
            .map(|(i, num)| Reverse((num, i)))
            .collect::<BinaryHeap<_>>();

        for _ in 0..k {
            if let Some(Reverse((num, i))) = bh.pop() {
                bh.push(Reverse((num * multiplier, i)));
            }
        }

        let mut vec = bh
            .into_iter()
            .map(|Reverse((num, i))| (i, num))
            .collect::<Vec<_>>();

        vec.sort_unstable();

        vec.into_iter().map(|(_, x)| x).collect()
    }
}
