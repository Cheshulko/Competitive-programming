// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers

struct Solution {}

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn check(mut x: i32) -> bool {
            while x > 0 {
                if x % 10 == 0 {
                    return false;
                }

                x /= 10;
            }

            true
        }

        for i in 1..n {
            if check(i) && check(n - i) {
                return vec![i, n - i];
            }
        }

        unreachable!()
    }
}
