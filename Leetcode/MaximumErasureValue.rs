// https://leetcode.com/problems/maximum-erasure-value

struct Solution {}

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        const N: usize = 10_000 + 1;

        let n = nums.len();

        let mut seen = vec![0; N];
        let mut ans = 0;

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + (nums[i] as usize);
        }

        let mut last = 0;
        for i in 0..n {
            let num = nums[i] as usize;
            last = last.max(seen[num]);
            seen[num] = i + 1;
            ans = ans.max(pref[i + 1] - pref[last]);
        }

        ans as i32
    }
}
