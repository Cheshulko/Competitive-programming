// https://leetcode.com/problems/determine-if-string-halves-are-alike

struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        const VOWELS: &[u8] = &[b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
        let n = s.len() / 2;

        let contains = |x: &&u8| VOWELS.iter().find(|y| x == y).is_some();
        s.as_bytes()[..=(n - 1)].iter().filter(contains).count()
            == s.as_bytes()[n..].iter().filter(contains).count()
    }
}
