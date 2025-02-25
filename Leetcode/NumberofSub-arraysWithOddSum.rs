// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum

struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: usize = 1_000_000_000 + 7;

        let mut ans = 0;

        let mut pref = 0;
        let mut cnt = [1, 0];
        for x in arr.into_iter() {
            pref += x as usize;

            cnt[pref & 1] += 1;

            ans += cnt[pref & 1 ^ 1];
            ans %= MOD;
        }

        ans as i32
    }
}
