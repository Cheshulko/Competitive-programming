// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters

struct Solution {}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        fn cnt_chars(str: String) -> Vec<i32> {
            str.chars()
                .map(|c| (c as u8) - b'a')
                .fold(vec![0; 26], |mut v, cur| {
                    v[(cur) as usize] += 1;
                    v
                })
        }
        let cnt = cnt_chars(chars);

        words
            .into_iter()
            .map(|word| (word.len() as i32, cnt_chars(word)))
            .filter_map(|(len, word)| {
                if cnt.iter().zip(word.iter()).all(|(f, s)| f >= s) {
                    Some(len)
                } else {
                    None
                }
            })
            .sum()
    }
}
