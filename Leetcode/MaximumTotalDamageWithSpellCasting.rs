// https://leetcode.com/problems/maximum-total-damage-with-spell-casting

struct Solution {}

impl Solution {
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.push(i32::MAX);
        power.sort_unstable();

        let mut prev = 0;
        let mut cnt = 1;
        let mut power_c = vec![];
        for p in power {
            let p = p as i64;
            if prev != p {
                power_c.push((prev, cnt));
                cnt = 0;
            }

            cnt += 1;
            prev = p;
        }

        let n = power_c.len();

        let mut ans = 0;
        let mut dp = vec![0; n];
        for i in 0..n {
            let (p, cnt) = power_c[i];

            dp[i] = dp[i].max(p * cnt);
            if i > 0 {
                let can = (p - power_c[i - 1].0 > 2) as i64;
                dp[i] = dp[i].max(dp[i - 1] + can * p * cnt);
            }
            if i > 1 {
                let can = (p - power_c[i - 2].0 > 2) as i64;
                dp[i] = dp[i].max(dp[i - 2] + can * p * cnt);
            }
            if i > 2 {
                assert!(p - power_c[i - 3].0 > 2);
                dp[i] = dp[i].max(dp[i - 3] + p * cnt);
            }

            ans = ans.max(dp[i]);
        }

        ans
    }
}
