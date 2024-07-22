// https://leetcode.com/problems/sort-the-people

struct Solution {}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut hn = heights
            .into_iter()
            .zip(names.into_iter())
            .collect::<Vec<_>>();

        hn.sort_unstable();

        hn.into_iter().rev().map(|x| x.1).collect::<Vec<_>>()
    }
}
