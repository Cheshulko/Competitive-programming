// https://leetcode.com/problems/smallest-integer-divisible-by-k

struct Solution {}

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let k = k as i64;

        let mut r = 0;
        for i in 1..=k {
            r = r * 10 + 1;
            r = r % k;
            if (r == 0) {
                return i as i32;
            }
        }

        -1
    }
}
