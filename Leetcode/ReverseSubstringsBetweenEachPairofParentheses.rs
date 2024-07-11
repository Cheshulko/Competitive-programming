// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses

struct Solution {}

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut s = s.into_bytes();

        let mut i = 0;
        loop {
            while i < s.len() && s[i] != b')' {
                i += 1;
            }
            if i == s.len() {
                break;
            }

            let mut j = i;
            while s[j] != b'(' {
                j -= 1;
            }
            (&mut s[j..=i]).reverse();

            i -= 1;
            s.remove(j);
            s.remove(i);
        }

        String::from_utf8(s).unwrap()
    }
}
