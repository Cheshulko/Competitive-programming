// https://leetcode.com/problems/maximum-number-of-words-you-can-type

struct Solution {}

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_letters = broken_letters.into_bytes();

        text.split(" ")
            .filter(|word| !word.as_bytes().iter().any(|b| broken_letters.contains(b)))
            .count() as i32
    }
}
