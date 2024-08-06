// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii

struct Solution {}

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        const NUMBERS: usize = 8;
        const ALPHABET: usize = 26;

        let mut freq = word
            .into_bytes()
            .into_iter()
            .fold(vec![0; ALPHABET], |mut f, ch| {
                f[(ch - b'a') as usize] += 1;
                f
            })
            .into_iter()
            .filter_map(|x| (x > 0).then_some(x))
            .collect::<Vec<_>>();

        freq.sort_unstable();
        freq.reverse();

        freq.chunks(NUMBERS).enumerate().fold(0, |acc, (i, group)| {
            acc + (i as i32 + 1) * group.iter().sum::<i32>()
        })
    }
}
