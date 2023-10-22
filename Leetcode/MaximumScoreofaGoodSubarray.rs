// https://leetcode.com/problems/maximum-score-of-a-good-subarray

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut s = BinaryHeap::<(Reverse<i32>, usize)>::new();
        let mut l = 0;
        let mut r = (n - 1) as i32;
        let mut ans = i32::MIN;

        for i in 0..n {
            s.push((Reverse(nums[i]), i));
        }

        while let Some(mn) = s.pop() {
            let ind = mn.1 as i32;
            let mn = mn.0 .0;

            if ind > r || ind < l {
                continue;
            }

            ans = ans.max((r - l + 1) * mn);
            match ind.cmp(&k) {
                Ordering::Greater => r = ind - 1,
                Ordering::Less => l = ind + 1,
                Ordering::Equal => break,
            }
        }

        ans
    }
}
