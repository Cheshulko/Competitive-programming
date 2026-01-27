// https://leetcode.com/problems/minimum-cost-path-with-edge-reversals

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
            use std::cmp::Reverse;
            use std::collections::BinaryHeap;

            let mut dist = vec![usize::MAX; adj.len()];
            dist[s] = 0;

            let mut visited = vec![false; adj.len()];
            let mut queue = BinaryHeap::new();

            queue.push(Reverse((0, s)));
            while let Some(Reverse((d, u))) = queue.pop() {
                if visited[u] {
                    continue;
                }

                visited[u] = true;
                for &(v, w) in &adj[u] {
                    if dist[v] > d + w {
                        dist[v] = d + w;
                        queue.push(Reverse((dist[v], v)));
                    }
                }
            }

            dist
        }

        let n = n as usize;
        let edges = edges.into_iter().fold(vec![vec![]; n], |mut edges, edge| {
            let &[u, v, w] = edge.as_slice() else {
                panic!()
            };
            let u = u as usize;
            let v = v as usize;
            let w = w as usize;

            edges[u].push((v, w));
            edges[v].push((u, 2 * w));

            edges
        });

        let dist = dijkstra(&edges, 0);

        let ans = dist[n - 1];
        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
