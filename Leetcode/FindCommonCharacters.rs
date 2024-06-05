// https://leetcode.com/problems/find-common-characters

struct Solution {}

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .map(|s| {
                s.into_bytes().into_iter().fold(vec![0; 26], |mut v, c| {
                    v[(c - b'a') as usize] += 1;
                    v
                })
            })
            .fold(vec![usize::MAX; 26], |u, v| {
                u.into_iter()
                    .zip(v.into_iter())
                    .map(|(a, b)| a.min(b))
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .enumerate()
            .flat_map(|(c, cnt)| (0..cnt).map(move |_| ((c as u8 + b'a') as char).to_string()))
            .collect::<Vec<_>>()
    }
}
