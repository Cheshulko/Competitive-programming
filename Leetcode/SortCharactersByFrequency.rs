// https://leetcode.com/problems/sort-characters-by-frequency

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut x = s
            .as_bytes()
            .into_iter()
            .fold(BTreeMap::new(), |mut btm, c| {
                *btm.entry(c).or_insert(0) += 1;
                btm
            })
            .into_iter()
            .collect::<Vec<_>>();
        x.sort_by(|a, b| b.1.cmp(&a.1));
        x.into_iter().fold(String::new(), |mut r, (c, fr)| {
            r.extend((*c as char).to_string().repeat(fr).chars());
            r
        })
    }
}
