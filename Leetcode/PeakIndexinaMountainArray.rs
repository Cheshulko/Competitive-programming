// https://leetcode.com/problems/peak-index-in-a-mountain-array

struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        fn bs(v: &[i32]) -> usize {
            match v.len() {
                1 => 0,
                2 if v[0] > v[1] => 0,
                2 if v[0] < v[1] => 1,
                n @ _ => {
                    let mid = n / 2;

                    match (v[mid - 1] < v[mid], v[mid] > v[mid + 1]) {
                        (true, true) => mid,
                        (true, false) => mid + bs(&v[mid..]),
                        (false, true) => bs(&v[..=mid]),
                        (false, false) => unreachable!(),
                    }
                }
            }
        }

        bs(&arr) as i32
    }
}
