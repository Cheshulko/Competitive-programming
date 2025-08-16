// https://leetcode.com/problems/maximum-69-number

struct Solution {}

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut p1 = 100000;
        let mut p2 = 10000;

        while p2 != 0 {
            let x = num % p1 / p2;
            if x == 6 {
                return num + 3 * p2;
            }
            p1 /= 10;
            p2 /= 10;
        }

        num
    }
}
