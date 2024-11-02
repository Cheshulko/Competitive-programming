// https://leetcode.com/problems/circular-sentence

struct Solution {}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split_whitespace().collect::<Vec<_>>();
        let n = words.len();
        for i in 0..n {
            let m = words[i].len();
            if words[i][m - 1..] != words[(i + 1) % n][..1] {
                return false;
            }
        }

        true
    }
}
