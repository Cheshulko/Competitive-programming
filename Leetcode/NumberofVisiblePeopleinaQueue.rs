// https://leetcode.com/problems/number-of-visible-people-in-a-queue

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut q: VecDeque<i32> = VecDeque::new();
        heights.iter().rev().for_each(|v| {
            let mut cnt = 0;

            while q.front().unwrap_or(&1_000_000) < v {
                q.pop_front();
                cnt += 1;
            }

            let x = if q.is_empty() { 0 } else { 1 };
            ans.push(cnt + x);
            q.push_front(*v);
        });
        ans.into_iter().rev().collect::<Vec<i32>>()
    }
}
