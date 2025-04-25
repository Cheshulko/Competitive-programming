// https://leetcode.com/problems/count-of-interesting-subarrays

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let n = nums.len();
        let k = k as i64;
        let modulo = modulo as i64;

        let nums = nums
            .into_iter()
            .map(|x| (x as i64 % modulo == k) as i64)
            .collect::<Vec<_>>();

        let mut cnt_pref = HashMap::new();
        cnt_pref.insert(0, 1);

        let mut ans = 0;
        let mut cnt = 0;
        for i in 0..nums.len() {
            cnt += nums[i];
            let rem = cnt % modulo;

            ans += cnt_pref.get(&((modulo + rem - k) % modulo)).unwrap_or(&0);
            *cnt_pref.entry(rem).or_default() += 1;
        }

        ans
    }
}
