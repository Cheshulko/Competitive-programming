// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock

struct Solution {}

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let n = prices.len();

        let mut ans = 0;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && prices[j - 1] - prices[j] == 1 {
                j += 1;
            }

            let cnt = j - i;
            ans += (cnt + 1) * cnt / 2;
            i = j;
        }

        ans as i64
    }
}
