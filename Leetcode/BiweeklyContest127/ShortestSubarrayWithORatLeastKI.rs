// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-i

struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = usize::MAX;

        for i in 0..n {
            let mut cur = 0;
            for j in i..n {
                cur |= nums[j];
                if cur >= k {
                    ans = ans.min(j - i + 1);
                }
            }
        }
        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
