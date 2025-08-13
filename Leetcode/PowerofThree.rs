// https://leetcode.com/problems/power-of-three

struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let n = n as i64;
        let mut cur = 1;
        while cur <= n {
            if cur == n {
                return true;
            }
            cur *= 3;
        }

        false
    }
}
