// https://leetcode.com/problems/vowels-game-in-a-string

struct Solution {}

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        const V: &[u8] = &[b'a', b'e', b'i', b'o', b'u'];

        s.into_bytes().into_iter().filter(|b| V.contains(b)).count() > 0
    }
}
