// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray

struct Solution {}

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        {
            let mut sum = 0;
            for &num in nums.iter() {
                sum += num;
                sum = sum.max(0);
                ans = ans.max(sum);
            }
        }
        {
            let mut sum = 0;
            for &num in nums.iter() {
                sum += num;
                sum = sum.min(0);
                ans = ans.max(sum.abs());
            }
        }

        ans
    }
}
