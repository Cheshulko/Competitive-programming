// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k

struct Solution {}

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let k = k as usize;

        let mut cnt = 0;
        let mut cur = 0;
        let mut p = 1;

        for c in s.chars().rev() {
            if c == '0' {
                cnt += 1;
                if p <= 1_000_000_000 {
                    p *= 2;
                }
            } else {
                if cur + p <= k {
                    cur += p;
                    cnt += 1;

                    if p <= 1_000_000_000 {
                        p *= 2;
                    }
                }
            }
        }

        cnt
    }
}
