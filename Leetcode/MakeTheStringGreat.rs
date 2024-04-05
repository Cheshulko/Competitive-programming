// https://leetcode.com/problems/make-the-string-great

struct Solution {}

impl Solution {
    pub fn make_good(s: String) -> String {
        s.chars()
            .fold(Vec::<char>::new(), |mut v, c| {
                if v.last().is_some_and(|last| {
                    let f1 = last.is_uppercase()
                        && c.is_lowercase()
                        && last.to_ascii_lowercase() == c.to_ascii_lowercase();

                    let f2 = last.is_lowercase()
                        && c.is_uppercase()
                        && last.to_ascii_lowercase() == c.to_ascii_lowercase();

                    f1 || f2
                }) {
                    v.pop();
                } else {
                    v.push(c)
                }

                v
            })
            .into_iter()
            .collect()
    }
}
