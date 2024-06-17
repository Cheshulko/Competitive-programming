// https://leetcode.com/problems/sum-of-square-numbers

struct Solution {}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        for i in 0..=(c as f64).sqrt().ceil() as i32 {
            let b2 = c - i * i;
            if b2 < 0 {
                continue;
            }
            let bs = (b2 as f64).sqrt().ceil() as i32;
            if i * i + bs * bs == c {
                return true;
            }
        }
        false
    }
}
