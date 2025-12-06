// https://leetcode.com/problems/count-partitions-with-max-min-difference-at-most-k

struct Solution {}

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BTreeSet;

        const M: usize = 1_000_000_000 + 7;

        let n = nums.len();

        let mut w = BTreeSet::new();
        let mut pref = vec![0; n + 1];
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        let mut j = 0;
        for (i, &num) in nums.iter().enumerate() {
            w.insert((num, i));
            pref[i + 1] = (pref[i] + dp[i]) % M;

            while let (Some(&(mi, _)), Some(&(ma, _))) = (w.first(), w.last()) {
                if ma - mi > k {
                    w.remove(&(nums[j], j));
                    j += 1;
                } else {
                    break;
                }
            }

            dp[i + 1] = (M + pref[i + 1] - pref[j]) % M;
        }

        dp[n] as i32
    }
}
