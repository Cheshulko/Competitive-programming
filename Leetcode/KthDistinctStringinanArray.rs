// https://leetcode.com/problems/kth-distinct-string-in-an-array

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let unq = arr
            .iter()
            .fold(BTreeMap::<&String, usize>::new(), |mut bt, s| {
                *bt.entry(s).or_default() += 1;
                bt
            });

        let mut cnt = 0;
        for s in arr.iter() {
            if unq.get(&s).unwrap() == &1 {
                cnt += 1;
            }
            if cnt == k {
                return s.clone();
            }
        }

        return String::new();
    }
}
use std::collections::BTreeMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let unq = arr
            .iter()
            .fold(BTreeMap::<&String, usize>::new(), |mut bt, s| {
                *bt.entry(s).or_default() += 1;
                bt
            });

        let mut cnt = 0;
        for s in arr.iter() {
            if unq.get(&s).unwrap() == &1 {
                cnt += 1;
            }
            if cnt == k {
                return s.clone();
            }
        }

        return String::new();
    }
}
