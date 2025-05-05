// https://leetcode.com/problems/domino-and-tromino-tiling

struct Solution {}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: usize = 1_000_000_000 + 7;

        let n = n as usize;

        let mut dp = vec![vec![0; 1 << 2]; n + 3];
        dp[0][0b11] = 1;
        for i in 0..=n + 1 {
            for prof in 0..1 << 2 {
                match prof {
                    0b00 => {
                        dp[i + 1][0b11] += dp[i][0b00];
                        dp[i + 1][0b11] %= MOD;

                        dp[i + 1][0b00] += dp[i][0b00];
                        dp[i + 1][0b00] %= MOD;

                        dp[i + 1][0b10] += dp[i][0b00];
                        dp[i + 1][0b10] %= MOD;

                        dp[i + 1][0b01] += dp[i][0b00];
                        dp[i + 1][0b01] %= MOD;
                    }
                    0b01 => {
                        dp[i + 1][0b10] += dp[i][0b01];
                        dp[i + 1][0b10] %= MOD;

                        dp[i + 1][0b11] += dp[i][0b01];
                        dp[i + 1][0b11] %= MOD;
                    }
                    0b10 => {
                        dp[i + 1][0b01] += dp[i][0b10];
                        dp[i + 1][0b01] %= MOD;

                        dp[i + 1][0b11] += dp[i][0b10];
                        dp[i + 1][0b11] %= MOD;
                    }
                    0b11 => {
                        dp[i + 1][0b00] += dp[i][0b11];
                        dp[i + 1][0b00] %= MOD;
                    }
                    _ => unreachable!(),
                }
            }
        }

        dp[n + 1][0b00] as i32
    }
}
