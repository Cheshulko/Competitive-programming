// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sums = HashMap::<i32, Vec<i32>>::new();

        for num in nums.into_iter() {
            let mut s = 0;
            let mut num1 = num;
            while num1 > 0 {
                s += num1 % 10;
                num1 /= 10;
            }

            sums.entry(s).or_default().push(num);
        }

        let mut ans = -1;
        for mut v in sums.into_values() {
            if v.len() > 1 {
                v.sort_unstable();
                ans = ans.max(v[v.len() - 1] + v[v.len() - 2]);
            }
        }

        ans
    }
}
