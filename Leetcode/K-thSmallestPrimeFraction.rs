// https://leetcode.com/problems/k-th-smallest-prime-fraction

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Wrap(pub f64);

impl Eq for Wrap {}

impl PartialOrd for Wrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Wrap {
    fn cmp(&self, other: &Wrap) -> Ordering {
        other.partial_cmp(self).unwrap()
    }
}

struct Solution {}

impl Solution {
    pub fn kth_smallest_prime_fraction(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize - 1;

        arr.sort_unstable();

        let mut brr = vec![];
        for i in 0..arr.len() {
            for j in (i + 1)..arr.len() {
                brr.push((Wrap(arr[i] as f64 / arr[j] as f64), (arr[i], arr[j])));
            }
        }

        brr.sort_unstable();

        vec![brr[k].1 .0, brr[k].1 .1]
    }
}
