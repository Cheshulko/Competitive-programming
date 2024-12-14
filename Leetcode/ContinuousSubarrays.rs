// https://leetcode.com/problems/continuous-subarrays

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans = 0;

        let mut i = 0;
        let mut j = 0;
        let mut len = 0;
        let mut min_heap = BinaryHeap::<(i32, usize)>::new();
        let mut max_heap = BinaryHeap::<(Reverse<i32>, usize)>::new();
        while i < n {
            if let (Some(&(min_value, _)), Some(&(Reverse(max_value), _))) =
                (min_heap.peek(), max_heap.peek())
            {
                if j == n || min_value.abs_diff(nums[j]) > 2 || max_value.abs_diff(nums[j]) > 2 {
                    ans += len;
                    len -= 1;

                    while let Some((_, ind)) = min_heap.peek() {
                        if *ind <= i {
                            min_heap.pop();
                        } else {
                            break;
                        }
                    }
                    while let Some((_, ind)) = max_heap.peek() {
                        if *ind <= i {
                            max_heap.pop();
                        } else {
                            break;
                        }
                    }
                    i += 1;
                } else {
                    min_heap.push((nums[j], j));
                    max_heap.push((Reverse(nums[j]), j));
                    len += 1;
                    j += 1;
                }
            } else {
                min_heap.push((nums[j], j));
                max_heap.push((Reverse(nums[j]), j));
                len += 1;
                j += 1;
            }
        }

        ans
    }
}
