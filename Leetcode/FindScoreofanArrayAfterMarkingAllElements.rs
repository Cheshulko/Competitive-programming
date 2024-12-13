// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let mut nums = nums.into_iter().map(|num| (num, false)).collect::<Vec<_>>();
        let mut bh = nums
            .iter()
            .enumerate()
            .map(|(ind, (num, _))| Reverse((*num, ind)))
            .collect::<BinaryHeap<_>>();

        let mut ans = 0;
        while let Some(Reverse((top, ind))) = bh.pop() {
            if nums[ind].1 {
                continue;
            }

            ans += top as i64;
            nums[ind].1 = true;
            if ind + 1 < n {
                nums[ind + 1].1 = true;
            }
            if ind > 0 {
                nums[ind - 1].1 = true;
            }
        }

        ans
    }
}
