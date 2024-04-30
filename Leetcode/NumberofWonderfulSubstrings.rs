// https://leetcode.com/problems/number-of-wonderful-substrings

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let n = word.len();

        let mut dp0 = [0; (1 << 10) + 1];
        let mut dp1 = [0; (1 << 10) + 1];

        let mut k = (0..2024_usize)
            .filter(|&x| x.count_ones() <= 1)
            .collect::<Vec<_>>();

        let mut ans = 0;
        for c in word.into_bytes().into_iter().map(|x| (x - b'a') as usize) {
            let c_bit = 1 << c;

            dp1[c_bit] += 1;

            for j in 0..1024 {
                dp1[j ^ c_bit] += dp0[j];
            }

            ans += k.iter().map(|&x| dp1[x]).sum::<usize>();

            std::mem::swap(&mut dp0, &mut dp1);
            dp1.fill(0);
        }

        ans as i64
    }
}
