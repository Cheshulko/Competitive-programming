// https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths

struct DSU {
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

struct Solution {}

impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        mut edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut ans = vec![false; queries.len()];

        edge_list.sort_by(|x, y| x[2].cmp(&y[2]));

        let mut queries_indexed = queries
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &Vec<i32>)>>();

        queries_indexed.sort_by(|x, y| x.1[2].cmp(&y.1[2]));

        let mut dsu = DSU::new(n as usize);
        let mut ind = 0;

        for q_ind in 0..queries_indexed.len() {
            while ind < edge_list.len() && edge_list[ind][2] < queries_indexed[q_ind].1[2] {
                dsu.union(edge_list[ind][0] as usize, edge_list[ind][1] as usize);
                ind += 1;
            }
            ans[queries_indexed[q_ind].0] = dsu.same(
                queries_indexed[q_ind].1[0] as usize,
                queries_indexed[q_ind].1[1] as usize,
            );
        }

        ans
    }
}
