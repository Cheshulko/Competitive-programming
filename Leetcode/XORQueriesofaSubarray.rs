// https://leetcode.com/problems/xor-queries-of-a-subarray

struct Solution {}

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let xors = arr.into_iter().fold(vec![0], |mut v, x| {
            v.push(v.last().unwrap() ^ x);
            v
        });

        queries
            .into_iter()
            .map(|x| xors[x[1] as usize + 1] ^ xors[x[0] as usize])
            .collect()
    }
}
