// https://leetcode.com/problems/count-total-number-of-colored-cells

struct Solution {}

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;

        1 + n * (n - 1) * 2
    }
}
