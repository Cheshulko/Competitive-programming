// https://leetcode.com/problems/redundant-connection

mod cm {
    use std::mem::swap;

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
                swap(&mut y, &mut x);
            }

            self.parents[y] = x;
            self.ranks[x] += self.ranks[y];

            true
        }
    }
}

struct Solution {}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut dsu = cm::DSU::new(n + 1);

        for v in edges.iter() {
            let (u, v) = (v[0] as usize, v[1] as usize);
            if !dsu.union(u, v) {
                return vec![u as i32, v as i32];
            }
        }

        unreachable!()
    }
}
