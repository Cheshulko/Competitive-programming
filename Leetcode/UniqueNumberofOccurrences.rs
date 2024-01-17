// https://leetcode.com/problems/unique-number-of-occurrences

struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let cnt: Vec<usize> = arr.into_iter().fold(vec![0; 2001], |mut v, x| {
            v[(x + 1000) as usize] += 1;
            v
        });
        let occurrences: Vec<usize> = cnt.into_iter().fold(vec![0; 1001], |mut v, x| {
            if x != 0 {
                v[x] += 1;
            }
            v
        });
        !occurrences.into_iter().any(|x| x > 1)
    }
}
