// https://leetcode.com/problems/special-array-ii

struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();

        let mut pref = vec![0; n + 1];
        for i in 1..n {
            pref[i] = pref[i - 1] + ((nums[i] & 1) ^ (nums[i - 1] & 1)) as usize
        }

        queries
            .into_iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize;

                (pref[r] - pref[l]) == r - l
            })
            .collect()
    }
}
