// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix

struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut x = mat
            .into_iter()
            .enumerate()
            .map(|(ind, v)| (v.iter().filter(|x| x == &&1).count(), ind as i32))
            .collect::<Vec<_>>();
        x.sort();
        x.iter().take(k as usize).map(|x| x.1).collect::<Vec<_>>()
    }
}
