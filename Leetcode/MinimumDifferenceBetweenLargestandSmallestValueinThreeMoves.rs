// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves

use std::collections::{BTreeMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut tm = BTreeMap::<i32, usize>::new();
        for x in nums.into_iter() {
            *tm.entry(x).or_default() += 1;
        }

        let min = tm.first_key_value().unwrap();
        let max = tm.last_key_value().unwrap();
        let mut ans = max.0 - min.0;

        let mut q = VecDeque::<(BTreeMap<i32, usize>, usize)>::new();
        q.push_back((tm, 3));

        while let Some((tm, k)) = q.pop_front() {
            if k == 0 || tm.is_empty() {
                if let (Some(min), Some(max)) = (tm.first_key_value(), tm.last_key_value()) {
                    ans = ans.min(max.0 - min.0);
                } else {
                    ans = 0;
                }
            } else {
                let mut tm1 = tm.clone();
                if let Some(mut min) = tm1.pop_first() {
                    min.1 -= 1;
                    if min.1 > 0 {
                        tm1.insert(min.0, min.1);
                    }

                    q.push_back((tm1, k - 1));
                }

                let mut tm2 = tm.clone();
                if let Some(mut max) = tm2.pop_last() {
                    max.1 -= 1;
                    if max.1 > 0 {
                        tm2.insert(max.0, max.1);
                    }

                    q.push_back((tm2, k - 1));
                }
            }
        }

        ans as i32
    }
}
