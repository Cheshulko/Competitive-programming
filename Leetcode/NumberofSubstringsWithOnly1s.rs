// https://leetcode.com/problems/number-of-substrings-with-only-1s

struct Solution {}

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let s = s.into_bytes();
        let n = s.len();

        let mut ans = 0;
        let mut i = 0;
        while i < n {
            if s[i] == b'0' {
                i += 1;
                continue;
            }
            let mut cnt = 0;
            while i < n && s[i] == b'1' {
                cnt += 1;
                i += 1;
            }
            ans += (cnt + 1) * cnt / 2;
            ans %= M;
        }

        ans as i32
    }
}
