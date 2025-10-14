// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-i

struct Solution {}

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let k = k as usize;

        let mut inc = vec![1; n];
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                inc[i] = inc[i - 1] + 1;
            }
            if inc[i] >= k && i >= k && inc[i - k] >= k {
                return true;
            }
        }

        false
    }
}
