// https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k

struct Solution {}

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;

        let mut pref = vec![0; n + 1];
        for (i, &num) in nums.iter().enumerate() {
            pref[i + 1] = pref[i] + num as i64;
        }

        let mut ma = i64::MIN;
        for i in 0..k {
            let mut j = i + k;
            if j > n {
                continue;
            }

            let mut best = 0;
            while j <= n {
                let next = pref[j] - pref[j - k];
                best = (best + next).max(next);

                ma = ma.max(best);
                j += k;
            }
        }

        ma
    }
}
