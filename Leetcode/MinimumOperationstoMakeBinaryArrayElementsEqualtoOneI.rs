// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i

struct Solution {}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut ans = 0;
        for i in 0..n - 2 {
            if nums[i] == 0 {
                ans += 1;
                nums[i] ^= 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
            }
        }

        if nums[n - 2] == 1 && nums[n - 1] == 1 {
            ans
        } else {
            -1
        }
    }
}
