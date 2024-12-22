// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet

mod cm_rmq {
    use std::{
        cmp::{max, min},
        ops::Range,
    };

    pub struct RMQ<T: Ord + Copy> {
        sparse_table: Vec<Vec<T>>,
        logs2: Vec<usize>,
        f: fn(T, T) -> T,
    }

    impl<T: Ord + Copy> RMQ<T> {
        fn new(f: fn(T, T) -> T, input: &[T]) -> RMQ<T> {
            RMQ {
                sparse_table: Self::build_sparse_table(f, input),
                logs2: vec![0]
                    .into_iter()
                    .chain((1..=input.len()).map(|x| x.ilog2() as usize))
                    .collect(),
                f: f,
            }
        }

        pub fn query(&self, range: Range<usize>) -> Option<T> {
            if range.is_empty()
                || (self.sparse_table.len() > 0 && self.sparse_table[0].len() < range.end)
            {
                return None;
            }
            let loglen = self.logs2[range.end - range.start];
            let idx: usize = range.end - (1 << loglen);
            let a = self.sparse_table[loglen][range.start];
            let b = self.sparse_table[loglen][idx];
            Some((self.f)(a, b))
        }

        fn build_sparse_table(f: fn(T, T) -> T, input: &[T]) -> Vec<Vec<T>> {
            let len = input.len();
            let mut sparse_table: Vec<Vec<T>> = vec![vec![]; len.ilog2() as usize + 1];

            for i in 0..input.len() {
                sparse_table[0].push(input[i]);
            }

            for i in 1..=len.ilog2() as usize {
                let mut j = 0;
                while j + (1 << i) <= input.len() {
                    let a = sparse_table[i - 1][j];
                    let b = sparse_table[i - 1][j + (1 << (i - 1))];
                    sparse_table[i].push(f(a, b));
                    j += 1;
                }
            }
            sparse_table
        }
    }

    impl<T: Ord + Copy> RMQ<T> {
        pub fn max(input: &[T]) -> RMQ<T> {
            RMQ::new(max, input)
        }
    }

    impl<T: Ord + Copy> RMQ<T> {
        pub fn min(input: &[T]) -> RMQ<T> {
            RMQ::new(min, input)
        }
    }
}

struct Solution {}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let rmq = cm_rmq::RMQ::max(&heights);
        let mut ans = vec![];
        'query: for q in queries.into_iter() {
            if q[0] == q[1] {
                ans.push(q[0]);
                continue 'query;
            }

            let ma_p = q[0].max(q[1]) as usize;
            let mi_p = q[0].min(q[1]) as usize;
            if heights[ma_p] > heights[mi_p] {
                ans.push(ma_p as i32);
                continue 'query;
            }

            let ma = heights[q[0] as usize].max(heights[q[1] as usize]);
            if rmq.query(ma_p..n).unwrap() <= ma {
                ans.push(-1);
            } else {
                let mut l = ma_p;
                let mut r = n;

                while r - l > 1 {
                    let m = (l + r) >> 1;
                    if rmq.query(l..m).unwrap() > ma {
                        r = m;
                    } else {
                        l = m;
                    }
                }
                ans.push(l as i32);
            }
        }

        ans
    }
}
