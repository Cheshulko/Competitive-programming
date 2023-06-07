// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c

struct Solution {}

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut ans = 0;
        while (a | b | c) != 0 {
            let al = a & 1;
            let bl = b & 1;
            let cl = c & 1;
            a >>= 1;
            b >>= 1;
            c >>= 1;
            if cl == 1 {
                ans += (al + bl == 0) as i32;
            } else {
                ans += al + bl;
            }
        }
        ans
    }
}
