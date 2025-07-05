// https://leetcode.com/problems/find-lucky-integer-in-an-array

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = BTreeMap::<i32, i32>::new();

        for num in arr.into_iter() {
            *freq.entry(num).or_default() += 1;
        }

        freq.into_iter()
            .rev()
            .filter_map(|(num, freq)| (num == freq).then_some(num))
            .next()
            .unwrap_or(-1)
    }
}
