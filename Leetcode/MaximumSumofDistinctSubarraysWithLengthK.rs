// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = nums.len();
        let mut have = HashSet::<i32>::new();
        let mut i = 0;
        let mut j = 0;

        let mut sum = 0;
        let mut ans = 0;

        while j <= n {
            if j - i >= k {
                if j - i == k {
                    ans = ans.max(sum);
                }
                have.remove(&nums[i]);
                sum -= nums[i] as i64;
                i += 1;
            }
            if j == n {
                break;
            }

            if have.contains(&nums[j]) {
                have.remove(&nums[i]);
                sum -= nums[i] as i64;
                i += 1;
            } else {
                have.insert(nums[j]);
                sum += nums[j] as i64;
                j += 1;
            }
        }

        ans
    }
}
