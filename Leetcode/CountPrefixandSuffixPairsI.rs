// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i

struct Solution {}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        fn z_fn(s: &[u8]) -> Vec<usize> {
            let n = s.len();
            let mut z = vec![0; n];
            let mut l = 0;
            let mut r = 0;

            for i in 1..n {
                if r >= i {
                    z[i] = z[i - l].min(r - i + 1);
                }
                while z[i] + i < n && s[z[i]] == s[z[i] + i] {
                    z[i] += 1;
                }
                if r < i + z[i] - 1 {
                    l = i;
                    r = i + z[i] - 1;
                }
            }

            z
        }

        let mut ans = 0;
        for (ind, word1) in words.iter().enumerate() {
            for word2 in words.iter().skip(ind + 1) {
                let word = format!("{}#{}", word1, word2);
                let word_z = z_fn(word.as_bytes());

                if word1.len() <= word2.len()
                    && word_z[word1.len() + 1] == word1.len()
                    && word_z[word.len() - word1.len()] == word1.len()
                {
                    ans += 1;
                }
            }
        }

        ans
    }
}
