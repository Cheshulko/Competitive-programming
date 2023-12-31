// https://leetcode.com/problems/largest-substring-between-two-equal-characters

struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        s.as_bytes()
            .into_iter()
            .enumerate()
            .fold(vec![vec![]; 26], |mut v, (ind, c)| {
                v[(c - b'a') as usize].push(ind as i32);
                v
            })
            .into_iter()
            .filter(|indexes| !indexes.is_empty())
            .map(|indexes| indexes.last().unwrap() - indexes.first().unwrap() - 1)
            .max()
            .unwrap_or(-1)
    }
}
