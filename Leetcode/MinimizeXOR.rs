// https://leetcode.com/problems/minimize-xor

struct Solution {}

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let cnt1 = num1.count_ones() as i32;
        let cnt2 = num2.count_ones() as i32;

        let mut ans = num1;
        for _ in 0..cnt1 - cnt2 {
            ans &= ans - 1;
        }
        for _ in 0..cnt2 - cnt1 {
            ans |= ans + 1;
        }

        ans
    }
}
