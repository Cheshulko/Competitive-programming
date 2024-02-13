// https://leetcode.com/problems/find-first-palindromic-string-in-the-array

struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|s| {
                let bytes = s.as_bytes();
                bytes.iter().zip(bytes.iter().rev()).all(|(a, b)| a == b)
            })
            .unwrap_or(String::new())
    }
}
