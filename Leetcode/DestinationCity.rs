// https://leetcode.com/problems/destination-city

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut hs = HashSet::<String>::new();
        let mut ans = String::new();

        let dests = paths
            .into_iter()
            .map(|mut path| {
                let dest = path.pop().unwrap();
                let from = path.pop().unwrap();
                hs.insert(from);
                dest
            })
            .collect::<Vec<_>>();

        for dest in dests.into_iter() {
            if !hs.contains(&dest) {
                return dest;
            }
        }

        unreachable!()
    }
}
