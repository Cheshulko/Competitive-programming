// https://leetcode.com/problems/maximum-score-words-formed-by-letters

struct Solution {}

impl Solution {
    fn calc(p: usize, words: &Vec<Vec<u8>>, letters_cnt: &Vec<usize>, score: &Vec<i32>) -> i32 {
        let n = words.len();

        let mut s = 0;
        let mut cnt = vec![0; 26];
        for b in 0..n {
            if p & (1 << b) > 0 {
                for &c in words[b].iter() {
                    cnt[c as usize] += 1;
                }
            }
        }

        for c in 0..26 {
            if cnt[c] > letters_cnt[c] {
                return 0;
            } else {
                s += (cnt[c] as i32) * score[c];
            }
        }

        s
    }

    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let n = words.len();

        let words = words
            .into_iter()
            .map(|s| {
                s.into_bytes()
                    .into_iter()
                    .map(|b| b - b'a')
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let letters_cnt =
            letters
                .into_iter()
                .map(|c| (c as u8) - b'a')
                .fold(vec![0; 26], |mut v, c| {
                    v[c as usize] += 1;
                    v
                });

        (1..(1 << n))
            .map(|p| Solution::calc(p, &words, &letters_cnt, &score))
            .max()
            .unwrap()
    }
}
