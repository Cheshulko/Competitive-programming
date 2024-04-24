// https://leetcode.com/problems/n-th-tribonacci-number

struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut t = [0, 1, 1];

        for _ in 3..=n {
            let x = t[0];
            t[0] = t[1];
            t[1] = t[2];
            t[2] = t[0] + t[1] + x;
        }

        if n < 3 {
            t[n]
        } else {
            t[2]
        }
    }
}
