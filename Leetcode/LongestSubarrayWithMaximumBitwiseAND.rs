// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and

struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();

        let mut ans = 0;
        let mut cnt = 0;
        let n = nums.len();

        for i in 0..n {
            if nums[i] == max {
                cnt += 1;
                ans = ans.max(cnt);
            } else {
                cnt = 0;
            }
        }

        ans
    }
}
