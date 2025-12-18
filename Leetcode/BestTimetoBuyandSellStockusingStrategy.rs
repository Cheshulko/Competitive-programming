// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-using-strategy

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = prices.len();

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + (strategy[i] * prices[i]) as i64;
        }

        let mut suf = vec![0; n + 1];
        for i in (0..n).rev() {
            suf[i] = suf[i + 1] + (strategy[i] * prices[i]) as i64;
        }

        let mut pref_prices = vec![0; n + 1];
        for i in 0..n {
            pref_prices[i + 1] = pref_prices[i] + prices[i] as i64;
        }

        let mut ans = pref[n];
        for i in 0..n - k + 1 {
            ans = ans.max(pref[i] + (pref_prices[i + k] - pref_prices[i + k / 2]) + suf[i + k]);
        }

        ans
    }
}
