// https://leetcode.com/problems/validate-stack-sequences

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut pushed: VecDeque<i32> = VecDeque::from(pushed);
        let mut popped: VecDeque<i32> = VecDeque::from(popped);
        let mut s: VecDeque<i32> = VecDeque::new();

        while !pushed.is_empty() {
            while !popped.is_empty() && s.back() == popped.front() {
                s.pop_back();
                popped.pop_front();
            }
            s.push_back(pushed.pop_front().unwrap());
        }
        while !popped.is_empty() && s.back() == popped.front() {
            s.pop_back();
            popped.pop_front();
        }
        popped.is_empty()
    }
}
