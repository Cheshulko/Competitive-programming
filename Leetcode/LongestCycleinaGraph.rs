// https://leetcode.com/problems/longest-cycle-in-a-graph

struct Graph {
    edges: Vec<i32>,
    data: Vec<(i32, i32, bool)>,
    ans: i32,
}

impl Graph {
    fn new(edges: Vec<i32>) -> Self {
        let cnt = edges.len();
        Graph {
            edges,
            data: vec![(0, 0, false); cnt],
            ans: -1,
        }
    }

    fn dfs(&mut self, cur: usize, cur_iter: &i32, depth: i32) {
        self.data[cur].2 = true;
        self.data[cur].1 = depth;
        self.data[cur].0 = *cur_iter;
        let to = self.edges[cur];
        if to != -1 {
            if self.data[to as usize].2 {
                if *cur_iter == self.data[to as usize].0 {
                    self.ans = std::cmp::max(self.ans, depth - self.data[to as usize].1 + 1);
                }
            } else {
                self.dfs(to as usize, cur_iter, depth + 1)
            }
        }
    }

    fn find(&mut self) -> i32 {
        let mut cur_iter = 1;
        for cur in 0..self.edges.len() {
            if !self.data[cur].2 {
                self.dfs(cur, &cur_iter, 0);
                cur_iter += 1;
            }
        }

        self.ans
    }
}

struct Solution {}

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut graph = Graph::new(edges);
        graph.find()
    }
}
