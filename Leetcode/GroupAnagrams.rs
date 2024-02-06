// https://leetcode.com/problems/group-anagrams

use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let strs = strs
            .into_iter()
            .map(|s| {
                let mut s_ = s.clone().as_bytes().to_vec();
                s_.sort();
                (s_, s)
            })
            .collect::<Vec<_>>();
        let uq = strs.iter().map(|x| &x.0).collect::<HashSet<_>>();
        let mut uq = uq.into_iter().fold(HashMap::new(), |mut hm, x| {
            hm.entry(x).or_insert(vec![]);
            hm
        });

        for (v, s) in strs.iter() {
            uq.entry(&v).and_modify(|y| y.push(s.clone()));
        }

        uq.values().map(|x| x.clone()).collect()
    }
}
