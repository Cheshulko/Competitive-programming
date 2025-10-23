//  https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i

struct Solution {}

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut s = s
            .into_bytes()
            .into_iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        let mut ns = vec![];

        while s.len() != 2 {
            ns.clear();
            for i in 0..s.len() - 1 {
                ns.push((s[i] + s[i + 1]) % 10);
            }

            std::mem::swap(&mut s, &mut ns);
        }

        s[0] == s[1]
    }
}
