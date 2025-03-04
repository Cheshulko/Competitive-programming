// https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three

struct Solution {}

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n as usize;
        let mut ma: usize = 3_usize.pow(18);

        while ma != 1 {
            if n >= ma {
                n -= ma;
            }

            ma /= 3;
        }

        n <= 1
    }
}
