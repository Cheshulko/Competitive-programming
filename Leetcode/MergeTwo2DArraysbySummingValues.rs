// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn merge_arrays(mut nums1: Vec<Vec<i32>>, mut nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        nums1.reverse();
        nums2.reverse();

        let mut res = vec![];
        while !nums1.is_empty() && !nums2.is_empty() {
            match nums1.last().unwrap()[0].cmp(&nums2.last().unwrap()[0]) {
                Ordering::Less => {
                    res.push(nums1.pop().unwrap());
                }
                Ordering::Greater => {
                    res.push(nums2.pop().unwrap());
                }
                Ordering::Equal => {
                    let x = nums1.pop().unwrap();
                    let y = nums2.pop().unwrap();
                    res.push(vec![x[0], x[1] + y[1]]);
                }
            }
        }

        while let Some(x) = nums1.pop() {
            res.push(x);
        }
        while let Some(x) = nums2.pop() {
            res.push(x);
        }

        res
    }
}
