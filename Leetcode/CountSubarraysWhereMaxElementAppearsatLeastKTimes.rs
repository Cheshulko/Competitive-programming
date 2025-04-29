// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times

struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let ma = nums.iter().max().copied().unwrap();

        let mut ans = 0;
        let mut cnt = 0;
        let mut l = 0;
        for r in 0..nums.len() {
            cnt += (nums[r] == ma) as i32;

            while l < r && cnt - (nums[l] == ma) as i32 >= k {
                cnt -= (nums[l] == ma) as i32;
                l += 1;
            }

            if cnt >= k {
                ans += l as i64 + 1;
            }
        }

        ans
    }
}
