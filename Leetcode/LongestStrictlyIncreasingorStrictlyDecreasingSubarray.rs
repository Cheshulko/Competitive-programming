// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut cur: i32 = 0;

        for w in nums.windows(2) {
            match w[1].cmp(&w[0]) {
                Ordering::Less if cur <= 0 => cur -= 1,
                Ordering::Less => cur = -1,
                Ordering::Greater if cur >= 0 => cur += 1,
                Ordering::Greater => cur = 1,
                Ordering::Equal => cur = 0,
            }

            ans = ans.max(1 + cur.abs());
        }

        ans
    }
}
