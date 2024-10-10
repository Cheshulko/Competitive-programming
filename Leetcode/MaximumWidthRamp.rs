// https://leetcode.com/problems/maximum-width-ramp

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        const MAX: usize = 5 * 10_000 + 1;

        let mut indx = vec![vec![]; MAX];
        for (i, x) in nums.into_iter().enumerate() {
            indx[x as usize].push(i);
        }

        let mut ans = 0;
        let mut t = BTreeSet::<usize>::new();
        let mut last_ind = 0;
        for x in 0..MAX {
            for &ind in indx[x].iter() {
                t.insert(ind);
                last_ind = ind;
            }

            if let Some(first) = t.first() {
                ans = ans.max(last_ind - first);
            }
        }

        ans as i32
    }
}
