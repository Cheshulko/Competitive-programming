// https://leetcode.com/problems/minimum-array-end

struct Solution {}

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let n = n as usize;
        let x = x as i64;

        let mut cur = x;
        for _ in 0..n - 1 {
            cur += 1;
            cur |= x;
        }

        cur
    }
}
