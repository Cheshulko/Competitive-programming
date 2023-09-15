// https://leetcode.com/problems/min-cost-to-connect-all-points

use std::cmp::Reverse;
use std::collections::BinaryHeap;

mod cm {
    pub struct DSU {
        parents: Vec<usize>,
        ranks: Vec<usize>,
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x != self.parents[x] {
                self.parents[x] = self.find(self.parents[x]);
            }

            self.parents[x]
        }

        pub fn same(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            x == y
        }

        pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            if x == y {
                return false;
            }

            if self.ranks[x] < self.ranks[y] {
                std::mem::swap(&mut y, &mut x);
            }

            self.parents[y] = x;
            self.ranks[x] += self.ranks[y];

            true
        }
    }
}

struct Solution {}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut tr = BinaryHeap::<Reverse<(i32, usize, usize)>>::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let p1 = &points[i];
                let p2 = &points[j];
                let d1 = (p1[0] - p2[0]).abs();
                let d2 = (p1[1] - p2[1]).abs();
                tr.push(Reverse((d1 + d2, i, j)));
            }
        }

        let mut ans = 0;
        let mut dsu = cm::DSU::new(n);

        while let Some(Reverse((d, i, j))) = tr.pop() {
            if !dsu.same(i, j) {
                dsu.union(i, j);

                ans += d;
            }
        }

        ans as i32
    }
}
