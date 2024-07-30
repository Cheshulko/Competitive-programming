// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced

struct Solution {}

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s = s.into_bytes();

        let mut pref_b = 0;
        let mut suf_a = s.iter().filter(|&x| *x == b'a').count();
        let mut ans = suf_a.min(s.len() - suf_a);

        for c in s.into_iter() {
            if c == b'a' {
                suf_a -= 1;
            } else {
                pref_b += 1;
            }

            ans = ans.min(pref_b + suf_a);
        }

        ans as i32
    }
}
