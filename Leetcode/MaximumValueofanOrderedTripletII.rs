// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii

struct Solution {}

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut suf_max = vec![];
        for &num in nums.iter().skip(2).rev() {
            suf_max.push(suf_max.last().copied().unwrap_or(0).max(num));
        }

        let mut pref_max = nums[0];
        let mut pref = i64::MIN;
        let mut ans = 0;

        for num in nums.into_iter().skip(1) {
            pref = pref.max(pref_max - num);
            pref_max = pref_max.max(num);

            if let Some(suf) = suf_max.pop() {
                ans = ans.max(pref * suf);
            }
        }

        ans
    }
}
