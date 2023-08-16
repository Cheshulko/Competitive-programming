// https://leetcode.com/problems/sliding-window-maximum

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut heap = BinaryHeap::<(i32, usize)>::new();
        let mut ans = vec![];

        for i in 0..=(k - 1) {
            heap.push((nums[i], i));
        }

        for i in (k - 1)..nums.len() {
            while let Some((_, ind)) = heap.peek() {
                if *ind < i - (k - 1) {
                    heap.pop();
                } else {
                    break;
                }
            }
            heap.push((nums[i], i));
            ans.push(heap.peek().unwrap().0);
        }

        ans
    }
}
