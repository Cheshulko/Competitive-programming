// https://leetcode.com/problems/delete-characters-to-make-fancy-string

struct Solution {}

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let s = s.into_bytes();
        let mut a = vec![];
        a.extend(s.iter().take(2));
        for c in s.windows(3) {
            if c[0] == c[1] && c[1] == c[2] {
            } else {
                a.push(c[2]);
            }
        }
        String::from_utf8(a).unwrap()
    }
}
