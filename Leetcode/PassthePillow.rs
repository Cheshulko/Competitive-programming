// https://leetcode.com/problems/pass-the-pillow

struct Solution {}

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let c = time / (n - 1);
        let h = time % (n - 1);

        if c % 2 == 1 {
            n - h
        } else {
            1 + h
        }
    }
}
