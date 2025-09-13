// https://leetcode.com/problems/find-most-frequent-vowel-and-consonant

struct Solution {}

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        s.into_bytes()
            .into_iter()
            .fold(vec![0; 26], |mut v, b| {
                v[(b - b'a') as usize] += 1;
                v
            })
            .into_iter()
            .enumerate()
            .fold([0, 0], |[mut ma1, mut ma2], (i, f)| {
                if [b'a', b'e', b'i', b'o', b'u'].contains(&(i as u8 + b'a')) {
                    ma1 = ma1.max(f);
                } else {
                    ma2 = ma2.max(f);
                }

                [ma1, ma2]
            })
            .into_iter()
            .sum()
    }
}
