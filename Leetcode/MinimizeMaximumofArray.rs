// https://leetcode.com/problems/minimize-maximum-of-array

struct Solution {}

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut sum = nums[0] as u64;
        for (ind, x) in nums[1..].iter().enumerate() {
            sum += *x as u64;
            let y = sum / (ind as u64 + 2) + if sum % (ind as u64 + 2) > 0 { 1 } else { 0 };
            ans = std::cmp::max(ans, y as i32);
        }
        ans
    }
}
