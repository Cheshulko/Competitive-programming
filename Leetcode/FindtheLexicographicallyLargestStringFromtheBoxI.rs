// https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-i

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }

        let n = word.len();
        let l = 1 + n - num_friends as usize;

        let mut bh = BinaryHeap::new();
        for i in 0..n {
            bh.push(&word[i..n.min(i + l)]);
        }

        bh.pop().unwrap().to_string()
    }
}
