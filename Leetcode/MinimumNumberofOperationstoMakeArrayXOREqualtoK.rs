// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut bit = 1;

        for _ in 0..32 {
            let k_bit = k & bit;

            let mut num_bit = 0;
            for &num in &nums {
                num_bit ^= num & bit;
            }

            if num_bit != k_bit {
                ans += 1;
            }

            bit <<= 1;
        }

        ans
    }
}
