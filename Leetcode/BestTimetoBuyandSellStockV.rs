// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-v

struct Solution {}

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = prices.len();

        let mut dp = vec![vec![[i64::MIN; 3]; k + 1]; n + 1];

        for ki in 0..=k {
            dp[0][ki][0] = 0;
        }

        for i in 0..n {
            let p = prices[i] as i64;

            for ki in 0..=k {
                dp[i + 1][ki][0] = dp[i + 1][ki][0].max(dp[i][ki][0]);
                dp[i + 1][ki][1] = dp[i + 1][ki][1].max(dp[i][ki][1]);
                dp[i + 1][ki][2] = dp[i + 1][ki][2].max(dp[i][ki][2]);

                if ki < k {
                    if dp[i][ki][0] != i64::MIN {
                        dp[i + 1][ki][1] = dp[i + 1][ki][1].max(dp[i][ki][0] - p);
                        dp[i + 1][ki][2] = dp[i + 1][ki][2].max(dp[i][ki][0] + p);
                    }

                    if dp[i][ki][1] != i64::MIN {
                        dp[i + 1][ki + 1][0] = dp[i + 1][ki + 1][0].max(dp[i][ki][1] + p);
                    }
                    if dp[i][ki][2] != i64::MIN {
                        dp[i + 1][ki + 1][0] = dp[i + 1][ki + 1][0].max(dp[i][ki][2] - p);
                    }
                }
            }
        }

        dp[n][k][0]
    }
}
