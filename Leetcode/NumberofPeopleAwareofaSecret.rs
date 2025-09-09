// https://leetcode.com/problems/number-of-people-aware-of-a-secret

struct Solution {}

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i64 = 1_000_000_000 + 7;
        let (n, delay, forget) = (n as usize, delay as usize, forget as usize);

        let mut dp: Vec<i64> = vec![0; 1000 * 2 + 2];
        let mut dp2: Vec<i64> = vec![0; 1000 * 2 + 2];
        for i in delay..forget {
            dp[i + 1] = 1;
        }
        for i in 0..forget {
            dp2[i + 1] = 1;
        }

        for i in 1..=n {
            for j in i..i + forget {
                dp2[j] += dp[i];
                dp2[j] %= MOD;
            }
            for j in i + delay..i + forget {
                dp[j] += dp[i];
                dp[j] %= MOD;
            }
        }

        dp2[n] as i32
    }
}
