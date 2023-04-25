// https://leetcode.com/problems/smallest-number-in-infinite-set

use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    s: BTreeSet<i32>,
    n: i32,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            s: BTreeSet::new(),
            n: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(value) = self.s.iter().next().copied() {
            if value < self.n {
                self.s.remove(&value);
                return value;
            } else {
                self.n += 1;
                return self.n - 1;
            }
        } else {
            self.n += 1;
            return self.n - 1;
        }
    }

    fn add_back(&mut self, num: i32) {
        if self.n > num {
            self.s.insert(num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */