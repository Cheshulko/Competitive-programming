// https://leetcode.com/problems/sum-of-all-subset-xor-totals

struct Solution {}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        for i in 1..(1 << n) {
            let mut x = 0;
            for j in 0..n {
                if i & (1 << j) > 0 {
                    x ^= nums[j];
                }
            }
            ans += x;
        }

        ans
    }
}
