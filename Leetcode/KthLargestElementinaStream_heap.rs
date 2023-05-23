// https://leetcode.com/problems/kth-largest-element-in-a-stream

use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort();
        Self {
            k: k as usize,
            heap: nums
                .into_iter()
                .rev()
                .map(Reverse)
                .take(k as usize)
                .collect(),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
