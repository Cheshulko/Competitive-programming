// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options

struct Solution {}

impl Solution {
    const MOD: i128 = 1_000_000_000 + 7;

    pub fn count_orders(n: i32) -> i32 {
        let n = (n as usize) - 1;
        let mut cur: i128 = 1;
        for x in 1..=n {
            let p = (2 * x + 1) as i128;
            cur = cur * (p + 1) * p / 2;
            cur %= Solution::MOD;
        }
        cur as i32
    }
}
