// https://leetcode.com/problems/buddy-strings

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let s: Vec<char> = s.chars().collect();
        let g: Vec<char> = goal.chars().collect();
        let p = s
            .iter()
            .zip(g.iter())
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|x| x.0)
            .collect::<Vec<_>>();

        let mut h1: HashMap<char, usize> = HashMap::new();
        let mut h2: HashMap<char, usize> = HashMap::new();
        for c in &s {
            *h1.entry(*c).or_insert(0) += 1;
        }
        for c in &g {
            *h2.entry(*c).or_insert(0) += 1;
        }
        for (k, v) in h1.iter() {
            if v > &1 && h2.get(k).unwrap_or(&0) > &1 && p.len() == 0 {
                return true;
            }
        }

        p.len() == 2 && s[p[0]] == g[p[1]] && s[p[1]] == g[p[0]]
    }
}
