// https://leetcode.com/problems/separate-black-and-white-balls

struct Solution {}

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut s = s.into_bytes();
        let n = s.len();

        let mut w = 0;
        let mut b = 0;
        let mut ans = 0;
        while w < n {
            while w < n && s[w] == b'1' {
                w += 1;
            }
            while b < n && s[b] == b'0' {
                b += 1;
            }
            if w == n {
                break;
            }
            if w < b {
                w += 1;
            } else {
                s.swap(w, b);
                ans += w - b;
                w += 1;
            }
        }

        ans as i64
    }
}
