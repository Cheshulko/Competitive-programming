// https://leetcode.com/problems/candy

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut ans = vec![0; n];

        let mut heap = BinaryHeap::<Reverse<(i32, usize)>>::from(
            ratings
                .iter()
                .enumerate()
                .map(|(a, b)| Reverse((*b, a)))
                .collect::<Vec<_>>(),
        );

        while let Some(mn) = heap.pop() {
            let rating = mn.0 .0;
            let index = mn.0 .1;

            let mut cur = 1;

            if index > 0 {
                let left = *ratings.get(index - 1).unwrap();
                if rating > left {
                    cur = cur.max(ans[index - 1] + 1)
                }
            }

            if index + 1 < n {
                let right = *ratings.get(index + 1).unwrap();
                if rating > right {
                    cur = cur.max(ans[index + 1] + 1)
                }
            }

            ans[index] = cur;
        }

        ans.iter().sum()
    }
}
