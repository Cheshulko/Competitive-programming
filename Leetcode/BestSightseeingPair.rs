// https://leetcode.com/problems/best-sightseeing-pair

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut bh = values
            .iter()
            .enumerate()
            .map(|(j, x)| (x - j as i32, j))
            .collect::<BinaryHeap<_>>();

        let mut ans = i32::MIN;
        for i in 0..values.len() {
            while let Some(&(_, j)) = bh.peek() {
                if j <= i {
                    bh.pop();
                } else {
                    break;
                }
            }

            if let Some(&(top, _)) = bh.peek() {
                ans = ans.max(values[i] + top + i as i32);
            }
        }

        ans
    }
}
