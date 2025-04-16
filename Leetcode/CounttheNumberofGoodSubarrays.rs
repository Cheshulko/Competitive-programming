// https://leetcode.com/problems/count-the-number-of-good-subarrays

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let n = nums.len();
        let mut hm = HashMap::<i32, i64>::new();

        let mut ans = 0;
        let mut cur = 0;

        let mut l = 0;
        for r in 0..n {
            if let Some(&v) = hm.get(&nums[r]) {
                cur -= v * (v - 1) / 2;
            }

            let v = hm.entry(nums[r]).or_default();
            *v += 1;
            cur += *v * (*v - 1) / 2;

            while cur >= k && l < r {
                let v = hm.entry(nums[l]).or_default();
                let mut ncur = cur;
                ncur -= *v * (*v - 1) / 2;

                let mut vv = *v;
                vv -= 1;
                ncur += vv * (vv - 1) / 2;

                if ncur >= k {
                    l += 1;
                    cur = ncur;
                    *v = vv;
                } else {
                    break;
                }
            }

            if cur >= k {
                ans += l + 1;
            }
        }

        ans as i64
    }
}
