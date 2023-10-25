// https://leetcode.com/problems/k-th-symbol-in-grammar

struct Solution {}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut k = k as usize;
        let mut p = 1 << (n - 1);
        let mut xr = 0;

        while p != 1 {
            if k > p >> 1 {
                k -= p >> 1;
                xr ^= 1;
            }
            p >>= 1;
        }

        xr
    }
}
