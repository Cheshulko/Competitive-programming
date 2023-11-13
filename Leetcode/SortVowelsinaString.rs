// https://leetcode.com/problems/sort-vowels-in-a-string

struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        const VOWELS: [u8; 10] = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
        let mut s_vowels = s
            .chars()
            .enumerate()
            .filter_map(|(ind, c)| {
                let c = c as u8;
                if VOWELS.contains(&c) {
                    Some((c, ind))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        s_vowels.sort();
        s_vowels.reverse();

        s.chars()
            .map(|c| {
                let cu8 = c as u8;
                if VOWELS.contains(&cu8) {
                    s_vowels.pop().unwrap().0 as char
                } else {
                    c
                }
            })
            .collect::<String>()
    }
}
