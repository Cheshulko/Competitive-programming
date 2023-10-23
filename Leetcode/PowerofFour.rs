// https://leetcode.com/problems/power-of-four

struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let x = (n as f64).log2();
        x.fract() == 0.0 && (x.trunc() as i32) % 2 == 0
    }
}
