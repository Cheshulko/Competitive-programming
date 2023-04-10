// https://leetcode.com/problems/valid-parentheses

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            match c {
                '(' => stack.push_back(c),
                '[' => stack.push_back(c),
                '{' => stack.push_back(c),
                ')' => {
                    if stack.back().unwrap_or(&'.') != &'(' {
                        return false;
                    } else {
                        stack.pop_back();
                    }
                }
                ']' => {
                    if stack.back().unwrap_or(&'.') != &'[' {
                        return false;
                    } else {
                        stack.pop_back();
                    }
                }
                '}' => {
                    if stack.back().unwrap_or(&'.') != &'{' {
                        return false;
                    } else {
                        stack.pop_back();
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}
