// https://leetcode.com/problems/reveal-cards-in-increasing-order

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();

        let mut ans = VecDeque::new();
        for d in deck.into_iter().rev() {
            if let Some(last) = ans.pop_back() {
                ans.push_front(last);
            }
            ans.push_front(d);
        }

        ans.into()
    }
}
