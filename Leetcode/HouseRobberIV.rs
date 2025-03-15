// https://leetcode.com/problems/house-robber-iv/description

struct Solution {}

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        fn can(v: Vec<usize>, k: usize) -> bool {
            if v.len() == 0 {
                return false;
            }

            let n = v.len();
            let mut dp = vec![0; n + 1];
            let mut ma = 1;
            dp[0] = 1;

            for i in 1..n {
                if v[i] - v[i - 1] > 1 {
                    dp[i] = dp[i].max(dp[i - 1] + 1);
                }
                if i > 1 {
                    dp[i] = dp[i].max(dp[i - 2] + 1);
                }
                ma = ma.max(dp[i]);
            }

            ma >= k
        }

        let k = k as usize;
        let mut l = nums.iter().min().unwrap() - 1;
        let mut r = *nums.iter().max().unwrap();

        while r - l > 1 {
            let m = (l + r) >> 1;
            let mut v = vec![];
            for i in 0..nums.len() {
                if nums[i] <= m {
                    v.push(i);
                }
            }

            if can(v, k) {
                r = m;
            } else {
                l = m;
            }
        }

        r
    }
}
