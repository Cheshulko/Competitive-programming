// https://leetcode.com/problems/product-of-array-except-self

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut pref = vec![1; n + 1];
        let mut suf = vec![1; n + 1];

        for (i, num) in nums.iter().enumerate() {
            pref[i + 1] = pref[i] * num;
        }

        for (i, num) in nums.into_iter().enumerate().rev() {
            suf[i] = suf[i + 1] * num;
        }

        (1..=n).map(|i| pref[i - 1] * suf[i]).collect()
    }
}
