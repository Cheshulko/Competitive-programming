// https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero

struct Solution {}

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let (mut num1, num2) = (num1 as i64, num2 as i64);

        fn bits(mut n: i64) -> i64 {
            let mut cnt = 0;
            while n > 0 {
                cnt += n & 1;
                n >>= 1;
            }
            return cnt;
        }

        for cnt in 0..61 {
            if num1 < cnt {
                continue;
            }
            if bits(num1) <= cnt {
                return cnt as i32;
            }
            num1 -= num2;
        }

        -1
    }
}
