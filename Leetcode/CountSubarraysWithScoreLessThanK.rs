// https://leetcode.com/problems/count-subarrays-with-score-less-than-k

struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let k = k as usize;
        let n = nums.len();
        let mut ans = 0;

        let mut len = 0;
        let mut sum = 0;
        let mut l = 0;
        for r in 0..n {
            sum += nums[r] as usize;
            len += 1;

            while l <= r && sum * len >= k {
                len -= 1;
                sum -= nums[l] as usize;
                l += 1;
            }

            ans += (r - l + 1) as i64;
        }

        ans
    }
}
