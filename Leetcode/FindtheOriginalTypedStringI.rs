// https://leetcode.com/problems/find-the-original-typed-string-i

struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let word = word.chars().collect::<Vec<_>>();

        let mut ans = 0;
        for i in 1..word.len() {
            if word[i - 1] == word[i] {
                ans += 1;
            }
        }

        ans + 1
    }
}
