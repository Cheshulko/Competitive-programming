// https://leetcode.com/problems/valid-word

struct Solution {}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let has_vowel =
            |s: &str| -> bool { s.chars().any(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')) };
        let has_consonant = |s: &str| -> bool {
            s.chars()
                .any(|c| !matches!(c, '0'..='9') && !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        };
        let not_any_else = |s: &str| -> bool {
            s.chars()
                .all(|c| matches!(c, 'a'..='z') || matches!(c, '0'..='9'))
        };

        if word.len() < 3 {
            return false;
        }

        let word = word.to_ascii_lowercase();
        has_vowel(&word) && has_consonant(&word) && not_any_else(&word)
    }
}
