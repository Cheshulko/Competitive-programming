// https://leetcode.com/problems/zero-array-transformation-iii

use std::cmp::Reverse;
use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let queries = queries
            .into_iter()
            .enumerate()
            .map(|(i, q)| (q[0] as usize, q[1] as usize, i))
            .collect::<Vec<_>>();

        let n = nums.len();
        let m = queries.len();
        let mut j = 0;

        let mut cnt = 0;
        let mut ans = 0;
        let mut takes = vec![false; m];

        let mut starts = vec![vec![]; n + 1];
        for q in queries.iter() {
            starts[q.0].push(q.2);
        }

        let mut ends = vec![vec![]; n + 1];
        for q in queries.iter() {
            ends[q.1 + 1].push(q.2);
        }

        let mut bt = BTreeSet::new();
        for i in 0..n {
            for &ind in ends[i].iter() {
                if takes[ind] {
                    cnt -= 1;
                }

                bt.remove(&(Reverse(queries[ind].1), ind));
            }

            for &ind in starts[i].iter() {
                bt.insert((Reverse(queries[ind].1), ind));
            }

            while j < m && cnt < nums[i] {
                if let Some((_, ind)) = bt.pop_first() {
                    cnt += 1;
                    ans += 1;
                    takes[ind] = true;
                } else {
                    return -1;
                }

                j += 1;
            }
            if cnt < nums[i] {
                return -1;
            }
        }

        (m - ans) as i32
    }
}
