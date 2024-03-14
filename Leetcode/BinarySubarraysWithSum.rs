// https://leetcode.com/problems/binary-subarrays-with-sum

struct Solution {}

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let goal = goal as usize;

        let mut ans = 0;
        let mut cnt = vec![0; 2 * (n + 1)];
        let mut pref_sum = vec![0; n + 1];

        cnt[0] = 1;

        for (i, num) in nums.into_iter().enumerate() {
            pref_sum[i + 1] = pref_sum[i] + num as usize;
            cnt[pref_sum[i + 1]] += 1;
        }

        for i in 0..(n + 1) {
            cnt[pref_sum[i]] -= 1;
            ans += cnt[goal + pref_sum[i]];
        }

        ans
    }
}
