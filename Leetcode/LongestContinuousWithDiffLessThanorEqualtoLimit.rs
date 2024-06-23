// https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();

        let mut min_bh = BinaryHeap::<(Reverse<i32>, usize)>::new();
        let mut max_bh = BinaryHeap::<(i32, usize)>::new();
        let mut out = vec![false; n];

        let mut ans = 0;
        let mut cur = 0;
        let mut i = 0;
        let mut j = 0;

        while i < n {
            while j < n {
                min_bh.push((Reverse(nums[j]), j));
                max_bh.push((nums[j], j));

                while let Some(&(Reverse(_), ind)) = min_bh.peek() {
                    if out[ind] {
                        min_bh.pop();
                    } else {
                        break;
                    }
                }

                while let Some(&(_, ind)) = max_bh.peek() {
                    if out[ind] {
                        max_bh.pop();
                    } else {
                        break;
                    }
                }

                let min = min_bh.peek().unwrap().0 .0;
                let max = max_bh.peek().unwrap().0;
                if max - min <= limit {
                    cur += 1;
                    j += 1;
                    ans = ans.max(cur);
                } else {
                    break;
                }
            }
            out[i] = true;
            i += 1;
            cur -= 1;
        }

        ans
    }
}
