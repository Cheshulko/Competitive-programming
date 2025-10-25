// https://leetcode.com/problems/calculate-money-in-leetcode-bank

struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        // 28 + 0*7 : 1 + 2 + .. + 7           = 28
        // 28 + 1*7 : 2 + 3 + .. + 8 = 28 + 7  = 35
        // 28 + 2*7                  = 28 + 14 = 42
        let a = (n / 7) * (28 + 28 + 7 * (n / 7 - 1)) / 2;
        let b = (n % 7) * (1 + ((n - 1) / 7) + ((n - 1) / 7) + n % 7) / 2;

        a + b
    }
}
