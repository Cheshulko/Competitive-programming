// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn minimum_length(mut s: String) -> i32 {
        let mut s = s.as_bytes().into_iter().collect::<VecDeque<_>>();
        while !s.is_empty() {
            if let (Some(l), Some(r)) = (s.front(), s.back()) {
                if l != r || s.len() == 1 {
                    break;
                }

                let x = l.clone();
                while s.front() == Some(&x) {
                    s.pop_front();
                }
                while s.back() == Some(&x) {
                    s.pop_back();
                }
            }
        }
        s.len() as i32
    }
}
