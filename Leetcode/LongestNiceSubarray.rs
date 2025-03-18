// https://leetcode.com/problems/longest-nice-subarray

struct Solution {}

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        const B: usize = 32;

        let n = nums.len();

        let mut ans = 1;
        let mut bits = [0; B];
        let mut l = 0;

        for r in 0..n {
            let num = nums[r];
            let mut can = true;
            for b in 0..B {
                bits[b] += (num & (1 << b) > 0) as usize;
                can &= bits[b] <= 1;
            }

            if can {
                ans = ans.max(r - l + 1);
            }

            while l <= r && !can {
                can = true;
                let num = nums[l];
                for b in 0..B {
                    bits[b] -= (num & (1 << b) > 0) as usize;
                    can &= bits[b] <= 1;
                }
                l += 1;
            }
        }

        ans as i32
    }
}
