// https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid

use std::collections::{BTreeSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let locked = locked
            .chars()
            .into_iter()
            .map(|x| x == '1')
            .collect::<Vec<_>>();

        let s = s.chars().collect::<Vec<_>>();

        let n = locked.len();
        if n & 1 == 1 {
            return false;
        }

        let mut q = vec![];
        let mut free = BTreeSet::new();
        let mut need = BTreeSet::new();

        for i in 0..n {
            if !locked[i] {
                free.insert(i);

                continue;
            }

            if s[i] == '(' {
                q.push(i);
            } else {
                if let Some(_) = q.pop() {
                } else {
                    need.insert(i);
                }
            }
        }

        for i in q.into_iter() {
            need.insert(i);
        }

        for i in need.into_iter() {
            if free.is_empty() {
                return false;
            }

            let mut to_remove = 0;
            if s[i] == '(' {
                if let Some(&x) = free.range(i..).next() {
                    to_remove = x;
                } else {
                    return false;
                }
            } else {
                if let Some(&x) = free.range(..i).next_back() {
                    to_remove = x;
                } else {
                    return false;
                }
            }

            free.remove(&to_remove);
        }

        return true;
    }
}
