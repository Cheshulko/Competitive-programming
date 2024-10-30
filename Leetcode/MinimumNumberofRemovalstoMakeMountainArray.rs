// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array

struct Solution {}

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut pref = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    pref[i] = pref[i].max(1 + pref[j]);
                }
            }
        }

        let mut suf = vec![0; n];
        for i in (0..n).rev() {
            for j in (i + 1..n).rev() {
                if nums[j] < nums[i] {
                    suf[i] = suf[i].max(1 + suf[j]);
                }
            }
        }

        let mut ans = n;
        for i in 1..(n - 1) {
            if pref[i] > 0 && suf[i] > 0 {
                ans = ans.min(n - 1 - pref[i] - suf[i]);
            }
        }

        ans as i32
    }
}
