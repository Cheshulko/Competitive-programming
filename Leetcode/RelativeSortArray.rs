// https://leetcode.com/problems/relative-sort-array

struct Solution {}

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut ind = vec![usize::MAX; 1001];
        for (i, x) in arr2.into_iter().enumerate() {
            ind[x as usize] = i;
        }

        arr1.sort_by(|a, b| match ind[*a as usize].cmp(&ind[*b as usize]) {
            std::cmp::Ordering::Equal => a.cmp(b),
            x => x,
        });

        arr1
    }
}
