// https://leetcode.com/problems/reverse-prefix-of-word

struct Solution {}

impl Solution {
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        if let Some(p) = word.find(ch) {
            let mut word = word.into_bytes();
            (&mut word[0..=p]).reverse();
            std::str::from_utf8(&word).unwrap().to_owned()
        } else {
            word
        }
    }
}
