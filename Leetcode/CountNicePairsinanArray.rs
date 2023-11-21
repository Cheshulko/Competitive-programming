// https://leetcode.com/problems/count-nice-pairs-in-an-array

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        // 24 + x = 42 + rev(x)
        // 24 - 42 = rev(x) - x
        fn rev(mut x: i64) -> i64 {
            let mut digits = vec![];
            while x > 0 {
                digits.push(x % 10);
                x /= 10;
            }
            digits
                .iter()
                .rev()
                .fold((0, 1), |(acc, p), x| (acc + p * x, p * 10))
                .0
        }

        let revs = nums.iter().map(|x| rev(*x)).collect::<Vec<_>>();
        let cnt = nums
            .iter()
            .enumerate()
            .fold(BTreeMap::<i64, i64>::new(), |mut cnt, (ind, x)| {
                let y = revs[ind] - *x;
                *cnt.entry(y).or_default() += 1;
                cnt
            });

        cnt.into_iter().fold(0, |acc, (_, x)| {
            if x > 0 {
                (acc + (x * (x - 1) / 2)) % (1_000_000_000 + 7)
            } else {
                acc
            }
        }) as i32
    }
}
