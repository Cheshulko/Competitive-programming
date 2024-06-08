// https://leetcode.com/problems/continuous-subarray-sum

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let n = nums.len();

        let mut hm = HashMap::<usize, bool>::new();
        hm.insert(0, true);

        let mut sum = nums[0] as usize;
        let mut prev = 0;
        for i in 1..n {
            prev = sum;
            sum += nums[i] as usize;

            let need = sum - (sum / k) * k;
            if hm.contains_key(&need) {
                return true;
            }

            let can = prev - (prev / k) * k;
            hm.insert(can, true);
        }

        false
    }
}
