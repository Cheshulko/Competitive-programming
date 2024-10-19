// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn solve(n: usize, k: usize) -> u8 {
        if n == 1 && k == 0 {
            return 0;
        }

        let len = (1 << n) - 1;
        match k.cmp(&(len / 2)) {
            Ordering::Less => Solution::solve(n - 1, k),
            Ordering::Greater => 1 ^ Solution::solve(n - 1, len - 1 - k),
            Ordering::Equal => 1,
        }
    }

    pub fn find_kth_bit(n: i32, k: i32) -> char {
        (b'0' + Solution::solve(n as usize, k as usize - 1)) as char
    }
}
