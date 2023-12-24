// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string

struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        (0..s.len())
            .fold(vec![vec![]; 2], |mut patterns, ind| {
                patterns[0].push((b'0' + (ind & 1) as u8) as char);
                patterns[1].push((b'0' + ((ind & 1) as u8) ^ 1) as char);
                patterns
            })
            .into_iter()
            .map(|pattern| {
                pattern
                    .into_iter()
                    .zip(s.chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .count()
            })
            .min()
            .unwrap() as i32
    }
}
