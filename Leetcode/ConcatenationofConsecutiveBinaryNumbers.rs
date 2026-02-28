// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers

struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let mut ans = 0;
        for i in 1..=n {
            let mut i = i as usize;

            let mut bs = vec![];
            while i > 0 {
                bs.push(i & 1);
                i >>= 1;
            }

            for b in bs.into_iter().rev() {
                ans <<= 1;
                ans += b;
                ans %= M;
            }
        }

        ans as i32
    }
}
