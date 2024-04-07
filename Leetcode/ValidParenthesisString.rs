// https://leetcode.com/problems/valid-parenthesis-string

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut open = VecDeque::new();
        let mut ast = VecDeque::new();

        for (i, c) in s.as_bytes().into_iter().enumerate() {
            match &c {
                b'(' => open.push_back(i),
                b'*' => ast.push_back(i),
                b')' => {
                    if !open.is_empty() {
                        open.pop_back();
                    } else if !ast.is_empty() {
                        ast.pop_front();
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        while !open.is_empty() && !ast.is_empty() {
            if open.front() < ast.front() {
                open.pop_front();
            }
            ast.pop_front();
        }

        open.is_empty()
    }
}
