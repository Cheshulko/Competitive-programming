// https://leetcode.com/problems/kth-largest-element-in-a-stream

use std::collections::BTreeSet;

struct KthLargest {
    k: usize,
    tree: BTreeSet<(i32, usize)>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        KthLargest {
            k: k as usize,
            tree: nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect(),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        let l = self.tree.len();
        self.tree.insert((val, l));
        self.tree.iter().rev().nth(self.k - 1).unwrap().0
    }
}
