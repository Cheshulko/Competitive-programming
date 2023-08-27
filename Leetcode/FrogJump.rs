// https://leetcode.com/problems/frog-jump

use std::collections::{BTreeMap, BTreeSet};

struct Solution {}

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();

        let mut m = BTreeMap::<&i32, usize>::new();
        for (ind, st) in stones.iter().enumerate() {
            m.insert(st, ind);
        }

        let mut can = vec![BTreeSet::<i32>::new(); n];
        can[0].insert(0);

        for (ind, st) in stones.iter().enumerate() {
            for c in can[ind].clone() {
                let tos = vec![*st + c - 1, *st + c, *st + c + 1]
                    .into_iter()
                    .filter(|x| x > &&0)
                    .collect::<Vec<_>>();

                for to in tos.into_iter() {
                    if let Some(ind_to) = m.get(&to) {
                        if *ind_to > ind {
                            can[*ind_to].insert(to - *st);
                        }
                    }
                }
            }
        }

        !can[n - 1].is_empty()
    }
}
