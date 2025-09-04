// https://leetcode.com/problems/find-closest-person

struct Solution {}

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        use std::cmp::Ordering;

        let d1 = z.abs_diff(x);
        let d2 = z.abs_diff(y);

        match d1.cmp(&d2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => 2,
        }
    }
}
