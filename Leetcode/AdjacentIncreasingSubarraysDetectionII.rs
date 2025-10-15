// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii

struct Solution {}

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut l = 1;
        let mut r = nums.len();

        while r - l > 1 {
            let m = (r + l) >> 1;
            if Solution::has_increasing_subarrays(&nums, m) {
                l = m;
            } else {
                r = m;
            }
        }

        l as i32
    }

    pub fn has_increasing_subarrays(nums: &Vec<i32>, k: usize) -> bool {
        let n = nums.len();

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
