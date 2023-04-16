// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary

struct Solution {}

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let mut dp: Vec<Vec<i64>> = vec![vec![0; 1001]; 1001];
        let modulo = 10_i64.pow(9) + 7;
        let mut cnt: Vec<Vec<i64>> = vec![vec![0; 1001]; 30];

        for word in &words {
            for (ind, ch) in word.chars().enumerate() {
                cnt[(ch as u8 - b'a') as usize][ind] += 1;
            }
        }

        let max_len = words.iter().map(|s| s.len()).max().unwrap();

        for i in 0..=max_len {
            dp[i][0] = 1;
        }

        for words_len in 1..=max_len {
            for target_len in 1..=target.len().min(words_len) {
                let target_char = target.chars().nth(target_len - 1).unwrap();

                dp[words_len][target_len] = (dp[words_len - 1][target_len - 1]
                    * cnt[(target_char as u8 - b'a') as usize][words_len - 1]
                    + dp[words_len - 1][target_len])
                    % modulo;
            }
        }
        dp[max_len][target.len()] as i32
    }
}
