// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades

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

type Edge = (i32, usize, usize, bool);

fn kruskal(n: usize, mut k: usize, mut edges: Vec<Edge>) -> Option<i32> {
    edges.sort_unstable();

    let mut mi = i32::MAX;
    let mut ans = vec![];
    let mut can = true;
    let mut dsu = cm::DSU::new(n);

    for &e @ (w, from, to, must) in edges.iter().rev() {
        if !must {
            continue;
        }

        if dsu.same(from, to) {
            can = false;
            break;
        }

        mi = mi.min(w);
        dsu.union(from, to);
        ans.push(e);
    }

    while let Some(e @ (_, from, to, must)) = edges.pop() {
        if must {
            continue;
        }

        if !dsu.same(from, to) {
            dsu.union(from, to);
            ans.push(e);
        }
    }

    for &(w, _, _, must) in ans.iter().rev() {
        if must {
            mi = mi.min(w);
        } else if k > 0 {
            k -= 1;
            mi = mi.min(2 * w);
        } else {
            mi = mi.min(w);
        }
    }

    (ans.len() == n - 1 && can).then_some(mi)
}

struct Solution;

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        let edges = edges
            .into_iter()
            .map(|v| (v[2], v[0] as usize, v[1] as usize, v[3] == 1))
            .collect::<Vec<_>>();

        kruskal(n, k, edges).unwrap_or(-1)
    }
}
