// https://leetcode.com/problems/partition-labels

mod cm {
    use std::mem::swap;

    pub struct DSU {
        parents: Vec<usize>,
        pub ranks: Vec<usize>,
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
    pub fn partition_labels(s: String) -> Vec<i32> {
        let n = s.len();
        let s = s
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        let mut dsu = cm::DSU::new(n + 1);

        for c in 0..26 {
            let p = s
                .iter()
                .enumerate()
                .filter_map(|(i, &x)| (x == c).then_some(i))
                .collect::<Vec<_>>();

            if let (Some(&mi), Some(&ma)) = (p.iter().min(), p.iter().max()) {
                for x in mi..ma {
                    dsu.union(x, ma);
                }
            }
        }

        let mut ans = vec![];
        let mut cur = dsu.find(0);
        let mut cnt = 0;
        for i in 0..=n {
            let y = dsu.find(i);
            if y == cur {
                cnt += 1;
            } else {
                ans.push(cnt);
                cnt = 1;
                cur = y;
            }
        }

        ans
    }
}
