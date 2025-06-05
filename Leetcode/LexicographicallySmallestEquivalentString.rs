// https://leetcode.com/problems/lexicographically-smallest-equivalent-string

use std::collections::{BTreeSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let tr = |s: String| -> Vec<usize> {
            s.into_bytes()
                .into_iter()
                .map(|b| (b - b'a') as usize)
                .collect::<Vec<_>>()
        };

        let col = |s: &Vec<usize>| -> Vec<Vec<usize>> {
            s.iter()
                .enumerate()
                .fold(vec![vec![]; 26], |mut ps, (i, &c)| {
                    ps[c].push(i);
                    ps
                })
        };

        let s1 = tr(s1);
        let s2 = tr(s2);
        let base_str = tr(base_str);

        let ps1 = col(&s1);
        let ps2 = col(&s2);

        let mut eq = vec![];
        let mut seen = vec![0; 26];

        let mut q = VecDeque::new();
        for c in 0..26 {
            if seen[c] == 0 {
                q.push_back(c as usize);
                seen[c] = eq.len() + 1;

                let mut hs = BTreeSet::new();
                while let Some(c) = q.pop_front() {
                    hs.insert(c);

                    for &to in ps1[c].iter() {
                        let to = s2[to];
                        if seen[to] == 0 {
                            seen[to] = eq.len() + 1;
                            q.push_back(to);
                        }
                    }
                    for &to in ps2[c].iter() {
                        let to = s1[to];
                        if seen[to] == 0 {
                            seen[to] = eq.len() + 1;
                            q.push_back(to);
                        }
                    }
                }

                eq.push(hs);
            }
        }

        base_str
            .into_iter()
            .map(|c| {
                let g = seen[c as usize];

                (b'a' + eq[g - 1].first().copied().unwrap() as u8) as char
            })
            .collect()
    }
}
