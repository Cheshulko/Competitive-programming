// https://leetcode.com/problems/sentence-similarity-iii

struct Solution {}

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut sentence1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let mut sentence2 = sentence2.split_whitespace().collect::<Vec<_>>();

        if sentence1.len() > sentence2.len() {
            std::mem::swap(&mut sentence1, &mut sentence2);
        }

        let mut i = 1;
        let mut j = sentence1.len();
        while i <= j && sentence1[i - 1] == sentence2[i - 1] {
            i += 1;
        }
        while i <= j && sentence1[j - 1] == sentence2[sentence2.len() - sentence1.len() + j - 1] {
            j -= 1;
        }

        i > j
    }
}
