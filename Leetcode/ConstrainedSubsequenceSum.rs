// https://leetcode.com/problems/constrained-subsequence-sum

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut b = BinaryHeap::<(i32, usize)>::new();
        let mut ans = i32::MIN;

        for i in 0..n {
            while let Some(top) = b.peek() {
                if i - top.1 > k {
                    b.pop();
                } else {
                    break;
                }
            }
            let mx = b.peek().unwrap_or(&(0, 0)).0;
            let mx = nums[i].max(mx + nums[i]);

            ans = ans.max(mx);
            b.push((mx, i));
        }

        ans
    }
}
