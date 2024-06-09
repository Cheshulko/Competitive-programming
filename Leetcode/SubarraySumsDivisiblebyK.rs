// https://leetcode.com/problems/subarray-sums-divisible-by-k

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut hm = HashMap::<i32, usize>::new();
        hm.insert(0, 1);
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..n {
            sum += nums[i];
            let mut need = sum % k;
            if need < 0 {
                need += k;
            }
            ans += *hm.entry(need).or_default();
            *hm.entry(need).or_insert(0) += 1;
        }

        ans as i32
    }
}
