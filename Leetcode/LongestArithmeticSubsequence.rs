// https://leetcode.com/problems/longest-arithmetic-subsequence

struct Solution {}

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![vec![1; 500 + 500 + 1]; nums.len()];
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                let d = (nums[i] - nums[j] + 500) as usize;
                cnt[i][d] = 1 + cnt[j][d];
                ans = ans.max(cnt[i][d]);
            }
        }
        ans
    }
}
