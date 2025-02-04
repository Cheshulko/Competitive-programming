// https://leetcode.com/problems/maximum-ascending-subarray-sum

struct Solution {}

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];

        let mut cur = nums[0];
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                cur += nums[i];
            } else {
                cur = nums[i];
            }
            ans = ans.max(cur);
        }

        ans
    }
}
