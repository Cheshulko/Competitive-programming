// https://leetcode.com/problems/find-smallest-letter-greater-than-target

struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        *letters
            .iter()
            .find(|c| c > &&target)
            .unwrap_or(letters.first().unwrap())
    }
}
