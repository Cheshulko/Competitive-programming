// https://leetcode.com/problems/find-the-pivot-integer

struct Solution {}

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        const EPS: f64 = 10e-6;

        let x = (n * n + n) / 2;
        let y = (x as f64).sqrt();
        if y.fract() < EPS {
            y as i32
        } else {
            -1
        }
    }
}
