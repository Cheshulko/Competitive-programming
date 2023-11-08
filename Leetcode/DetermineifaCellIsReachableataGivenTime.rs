// https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time

struct Solution {}

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let dx = (fx - sx).abs();
        let dy = (fy - sy).abs();
        let m = dx.max(dy);
        (m != 0 && t >= m) || (m == 0 && t != 1)
    }
}
