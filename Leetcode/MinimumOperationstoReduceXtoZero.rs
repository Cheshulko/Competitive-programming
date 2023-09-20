// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut bt = BTreeMap::<i32, usize>::new();
        bt.insert(0, 0);

        let mut ans = usize::MAX;
        let mut sum = 0;
        for (ind, num) in nums.iter().enumerate() {
            sum += *num;
            if sum == x {
                ans = ans.min(ind + 1);
            }
            bt.insert(sum, ind + 1);
        }

        let n = nums.len();

        let mut sum = 0;
        for (ind, num) in nums.into_iter().rev().enumerate() {
            sum += num;
            let need = x - sum;
            if need < 0 {
                break;
            }

            if let Some(v) = bt.get(&need) {
                if v + ind < n {
                    ans = ans.min(ind + 1 + *v);
                }
            }
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
