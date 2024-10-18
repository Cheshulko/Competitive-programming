// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets

struct Solution {}

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 1 << 18];

        let mut ans = 0;
        for mask in 0..(1 << nums.len()) {
            let mut s = 0;
            for b in 0..16 {
                if mask & (1 << b) > 0 {
                    s |= nums[b];
                }
            }
            cnt[s as usize] += 1;
            ans = ans.max(cnt[s as usize]);
        }

        ans
    }
}
