// https://leetcode.com/problems/2-keys-keyboard

struct Solution {}

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = vec![vec![usize::MAX; 2]; 2 * n + 2];
        dp[1][0] = 0;
        dp[1][1] = 1;

        for i in 2..=n {
            let mut j = 1;
            while j * j <= i {
                if i % j == 0 {
                    if dp[j][0] != usize::MAX {
                        dp[i][0] = dp[i][0].min(dp[j][0] + i / j);
                    }
                    if dp[j][1] != usize::MAX {
                        dp[i][0] = dp[i][0].min(dp[j][1] + i / j - 1);
                    }

                    if dp[i / j][0] != usize::MAX {
                        dp[i][0] = dp[i][0].min(dp[i / j][0] + j);
                    }
                    if dp[i / j][1] != usize::MAX {
                        dp[i][0] = dp[i][0].min(dp[i / j][1] + j - 1);
                    }

                    dp[i][1] = dp[i][1].min(dp[i][0] + 1);
                }
                j += 1;
            }
        }

        dp[n][0] as i32
    }
}
