// https://leetcode.com/problems/rank-transform-of-an-array

use std::collections::{BTreeSet, HashMap};

struct Solution {}

impl Solution {
    fn compress(input: Vec<i32>) -> Vec<i32> {
        let mut hs = BTreeSet::<i32>::new();
        let mut mp = HashMap::<i32, i32>::new();

        for &x in input.iter() {
            hs.insert(x);
        }
        for x in hs.into_iter() {
            if !mp.contains_key(&x) {
                let len = mp.len() as i32 + 1;
                mp.insert(x, len);
            }
        }

        let mut res = vec![];
        for x in input.into_iter() {
            res.push(*mp.get(&x).unwrap());
        }

        res
    }

    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        Solution::compress(arr)
    }
}
