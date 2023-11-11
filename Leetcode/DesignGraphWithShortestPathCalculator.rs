// https://leetcode.com/problems/design-graph-with-shortest-path-calculator

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Graph {
    n: usize,
    graph: Vec<Vec<(usize, i32)>>, // to, weight
}

impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let graph = edges.into_iter().fold(vec![vec![]; n], |mut v, x| {
            let from = x[0] as usize;
            let to = x[1] as usize;
            let weight = x[2];
            v[from].push((to, weight));
            v
        });

        Graph { n, graph }
    }

    pub fn add_edge(&mut self, edge: Vec<i32>) {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        let weight = edge[2];
        self.graph[from].push((to, weight));
    }

    pub fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        if node1 == node2 {
            0
        } else {
            self.dijkstra(node1 as usize)[node2 as usize]
                .unwrap_or((0, -1))
                .1
        }
    }

    fn dijkstra(&self, start: usize) -> Vec<Option<(usize, i32)>> {
        let mut ans = vec![None; self.n];
        let mut prio = BinaryHeap::new();

        for (new, weight) in &self.graph[start] {
            ans[*new] = Some((start, *weight));
            prio.push((Reverse(*weight), new, start));
        }

        while let Some((dist_new, new, prev)) = prio.pop() {
            let dist_new = dist_new.0;
            if let Some((p, d)) = ans[*new] {
                if p == prev && d == dist_new {
                } else {
                    continue;
                }
            } else {
                continue;
            }

            for (next, weight) in &self.graph[*new] {
                if let Some((_, dist_next)) = ans[*next] {
                    if dist_new + *weight > dist_next {
                    } else {
                        ans[*next] = Some((*new, *weight + dist_new));
                        prio.push((Reverse(*weight + dist_new), next, *new));
                    }
                } else {
                    ans[*next] = Some((*new, *weight + dist_new));
                    prio.push((Reverse(*weight + dist_new), next, *new));
                }
            }
        }

        ans
    }
}
