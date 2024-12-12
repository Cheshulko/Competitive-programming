// https://leetcode.com/problems/take-gifts-from-the-richest-pile

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut bh = gifts
            .into_iter()
            .map(|x| x as i64)
            .collect::<BinaryHeap<_>>();

        for _ in 0..k {
            if bh.is_empty() {
                break;
            }
            let cur = bh.pop().unwrap();
            let take = (cur as f64).sqrt() as i64;
            bh.push(take);
        }

        let mut ans = 0;
        while let Some(cur) = bh.pop() {
            ans += cur;
        }

        ans
    }
}
