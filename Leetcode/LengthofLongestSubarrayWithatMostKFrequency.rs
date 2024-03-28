// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();

        let mut i = 0;
        let mut j = 0;

        let mut ans = 0;
        let mut cnt = 0;

        let mut hm = HashMap::<i32, usize>::new();

        while i < n {
            while j < n && hm.get(&nums[j]).unwrap_or(&0) + 1 <= k {
                *hm.entry(nums[j]).or_insert(0) += 1;
                cnt += 1;
                j += 1;
            }

            if i != j {
                hm.entry(nums[i]).and_modify(|x| *x -= 1);
            }

            ans = ans.max(cnt);
            cnt -= 1;
            cnt = cnt.max(0);

            i += 1;
            j = j.max(i);
        }

        ans
    }
}
