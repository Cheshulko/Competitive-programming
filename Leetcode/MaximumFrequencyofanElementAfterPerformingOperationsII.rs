// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii

struct Solution {}

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        use std::collections::{BTreeMap, HashSet};

        nums.sort_unstable();
        let num_operations = num_operations as usize;

        let nums_initial =
            nums.clone()
                .into_iter()
                .fold(BTreeMap::<i32, usize>::new(), |mut bt, num| {
                    *bt.entry(num).or_default() += 1;
                    bt
                });

        let nums_unique = nums
            .clone()
            .into_iter()
            .fold(HashSet::<i32>::new(), |mut hs, num| {
                hs.insert(num - k);
                hs.insert(num);
                hs.insert(num + k);

                hs
            });

        let mut ans = 0;
        for &i in nums_unique.iter() {
            let l = nums.partition_point(|&num| num + k < i);
            let r = nums.partition_point(|&num| num - k <= i);
            let initial = nums_initial.get(&i).copied().unwrap_or(0);
            let cnt = (r - l - initial).min(num_operations);

            ans = ans.max(initial + cnt);
        }

        ans as i32
    }
}
