// https://leetcode.com/problems/calculate-money-in-leetcode-bank

struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let weeks = n / 7;
        let current_day = n % 7;

        let x1 = 28 * weeks;
        let x2 = 7 * weeks * (weeks - 1) / 2;
        let x3 = weeks * current_day + (current_day * (current_day + 1) / 2);

        x1 + x2 + x3
    }
}
