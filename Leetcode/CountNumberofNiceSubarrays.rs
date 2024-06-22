// https://leetcode.com/problems/count-number-of-nice-subarrays

struct Solution {}

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut i = 0;
        let mut j = 0;

        let mut cnt = 0;
        let mut ans = 0;
        while i < n {
            while j < n && cnt + nums[j] % 2 < k {
                cnt += nums[j] % 2;
                j += 1;
            }
            let mut a = 0;
            while j < n && cnt + nums[j] % 2 == k {
                cnt += nums[j] % 2;
                a += 1;
                j += 1;
            }
            let mut b = 1;
            while cnt - nums[i] % 2 == k {
                b += 1;
                i += 1;
            }
            ans += a * b;
            cnt -= nums[i] % 2;
            i += 1;
        }

        ans
    }
}
