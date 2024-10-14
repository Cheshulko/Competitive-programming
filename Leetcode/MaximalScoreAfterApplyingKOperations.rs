// https://leetcode.com/problems/maximal-score-after-applying-k-operations

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<BinaryHeap<_>>();

        let mut ans = 0;
        for _ in 0..(k as usize) {
            let (x, i) = nums.pop().unwrap();
            ans += x as i64;

            nums.push(((x + 2) / 3, i));
        }

        ans
    }
}
